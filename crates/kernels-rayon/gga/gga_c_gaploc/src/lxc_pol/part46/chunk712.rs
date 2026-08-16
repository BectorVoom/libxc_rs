//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 712/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk712(t1020: f64, t3113: f64, t12555: f64, t12558: f64, t12566: f64, t12569: f64, t12580: f64, t13087: f64, t13088: f64) -> f64 {
    let t13089 = t1020 * t3113;
    let t13091 = 9.0_f64 / 256.0_f64 * t12555;
    let t13092 = 9.0_f64 / 8192.0_f64 * t12558;
    let t13093 = 3.0_f64 / 8192.0_f64 * t12566;
    let t13094 = 3.0_f64 / 256.0_f64 * t12569;
    let t13095 = 2.0_f64 * t12580;
    let t13096 = t13087 + t13088 - t13089 / 2.0_f64 - t13091 - t13092 + t13093 + t13094 + t13095;
    t13096
}
