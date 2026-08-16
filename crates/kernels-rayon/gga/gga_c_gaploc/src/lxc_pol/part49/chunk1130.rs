//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1130/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1130(t13847: f64, t2684: f64, t7354: f64, t43783: f64, t43787: f64, t43790: f64, t43793: f64, t43800: f64, t43803: f64, t43806: f64, t43809: f64, t43812: f64, t43815: f64, t43817: f64) -> f64 {
    let t47389 = t2684 * t7354 * t13847;
    let t47394 = -t43783 - 0.25561950635947166451e0_f64 * t47389 - t43787 + t43790 + t43793 + t43800 - t43803 + t43806 - t43809 - 0.14896037479937677779e-1_f64 * t43812 + 0.46011511144704899612e1_f64 * t43815 - 0.14896037479937677779e-1_f64 * t43817;
    t47394
}
