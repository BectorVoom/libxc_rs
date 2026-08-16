//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 530/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk530(t7218: f64, t2164: f64, t356: f64, t638: f64, t639: f64, t1276: f64, t640: f64, t1173: f64, t205: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7219 = 0.15243824895787514157e-3_f64 * t7218;
    let t7220 = t2164 * t356;
    let t7222 = t638 * t639 * t7220;
    let t7223 = 0.30487649791575028314e-3_f64 * t7222;
    let t7224 = t640 * t1276;
    let t7226 = t638 * t639 * t7224;
    let t7227 = 0.15243824895787514157e-3_f64 * t7226;
    let t7228 = t1173 * t205;
    let t7229 = t671 * t7228;
    (t7219, t7220, t7223, t7224, t7227, t7228, t7229)
}
