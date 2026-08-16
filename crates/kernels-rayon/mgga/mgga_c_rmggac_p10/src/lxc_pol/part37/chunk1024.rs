//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1024/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1024(t78560: f64, t2231: f64, t2338: f64, t638: f64, t639: f64, t2164: f64, t2474: f64, t76521: f64, t76524: f64, t76527: f64, t72145: f64, t72147: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78561 = 0.15243824895787514157e-3_f64 * t78560;
    let t78566 = t638 * t639 * t2338 * t2231;
    let t78567 = 0.15243824895787514157e-3_f64 * t78566;
    let t78570 = t638 * t639 * t2164 * t2474;
    let t78571 = 0.15243824895787514157e-3_f64 * t78570;
    let t78574 = 0.30487649791575028312e-3_f64 * t76521;
    let t78575 = 0.16263363996404810741e-4_f64 * t76524;
    let t78576 = 0.16263363996404810741e-4_f64 * t76527;
    let t78577 = 0.27274661654245341729e-1_f64 * t72145;
    let t78578 = 0.36366215538993788972e-1_f64 * t72147;
    (t78561, t78567, t78571, t78574, t78575, t78576, t78577, t78578)
}
