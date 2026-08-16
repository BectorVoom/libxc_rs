//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 826/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk826(t10782: f64, t2464: f64, t2465: f64, t2684: f64, t13072: f64, t32757: f64, t25359: f64, t2615: f64, t9438: f64, t41448: f64, t41477: f64, t2344: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44128 = t2684 * t2464 * t2465 * t10782;
    let t44130 = t32757 * t13072;
    let t44133 = t2615 * t9438 * t25359;
    let t44147 = 0.31952438294933958063e0_f64 * t41448;
    let t44157 = 0.12780975317973583225e0_f64 * t41477;
    let t44255 = t550 * t2344;
    (t44128, t44130, t44133, t44147, t44157, t44255)
}
