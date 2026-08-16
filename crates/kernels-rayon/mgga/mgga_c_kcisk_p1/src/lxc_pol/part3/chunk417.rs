//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 417/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk417(t116: f64, t3139: f64, t3138: f64, t979: f64, t142: f64, t181: f64, t15: f64, t163: f64, t167: f64, t196: f64, t183: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3140 = t116 * t3139;
    let t3141 = t3138 * t3140;
    let t3142 = t979 * t3141;
    let t3144 = t142 * t181;
    let t3148 = t163 * t15;
    let t3155 = t196 * t167;
    let t3156 = t816 * t183;
    (t3140, t3141, t3142, t3144, t3148, t3155, t3156)
}
