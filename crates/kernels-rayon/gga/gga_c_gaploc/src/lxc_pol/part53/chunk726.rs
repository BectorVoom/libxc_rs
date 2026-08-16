//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 726/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk726(t12539: f64, t12948: f64, t12952: f64, t12955: f64, t12958: f64, t13789: f64, t13793: f64, t13795: f64, t13796: f64, t13798: f64, t13802: f64, t13806: f64) -> f64 {
    let t14472 = 0.29792074959875355558e-1_f64 * t13789 - 0.29792074959875355558e-1_f64 * t13793 - t12948 + t13795 - t13796 - 0.76685851907841499353e0_f64 * t12539 + t12952 - t12955 - 0.76685851907841499352e0_f64 * t12958 + 0.71500979903700853338e0_f64 * t13798 - 0.92023022289409799224e1_f64 * t13802 + 0.23005755572352449806e2_f64 * t13806;
    t14472
}
