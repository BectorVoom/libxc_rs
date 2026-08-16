//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1060/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1060(t222: f64, t227: f64, t12830: f64, t12924: f64, t15772: f64, t15775: f64, t224: f64, t3283: f64, t3288: f64, t1060: f64, t3289: f64, t10441: f64, t10449: f64, t229: f64, t3293: f64, zeta_threshold: f64) -> (f64, f64) {
    let t223 = t222 <= zeta_threshold;
    let t228 = t227 <= zeta_threshold;
    let t15781 = piecewise3(t223, 0.0_f64, -8.0_f64 / 27.0_f64 * t15772 * t12830 + 4.0_f64 / 3.0_f64 * t15775 * t3283 + 4.0_f64 / 3.0_f64 * t224 * t12924);
    let t15783 = 1.0_f64 / t3288 / t227;
    let t15786 = t3289 * t1060;
    let t15792 = piecewise3(t228, 0.0_f64, -8.0_f64 / 27.0_f64 * t15783 * t10441 + 4.0_f64 / 3.0_f64 * t15786 * t3293 + 4.0_f64 / 3.0_f64 * t229 * t10449);
    (t15781, t15792)
}
