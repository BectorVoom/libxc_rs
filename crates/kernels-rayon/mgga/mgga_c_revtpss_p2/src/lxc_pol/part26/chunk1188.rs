//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1188/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1188(t213: f64, t26473: f64, t10982: f64, t2061: f64, t9646: f64, t25365: f64, t26544: f64, t93190: f64, t95726: f64, t10638: f64, t10978: f64, t231: f64, t25383: f64, t26441: f64, t2645: f64, t26547: f64, t2772: f64, t7070: f64, t7076: f64, t7398: f64, t7403: f64, t887: f64, t95866: f64, t95872: f64, t95876: f64, t95888: f64, t95891: f64, t95893: f64) -> f64 {
    let t95894 = t213 * t26473;
    let t95899 = 0.19637199382202157274e-3_f64 * t9646 * t2061 * t10982;
    let t95900 = t25365 * t26544;
    let t95902 = t93190 * t95726;
    let t95904 = 0.29272321618148349057e-1_f64 * t95866 + 0.39512695097613069591e1_f64 * t26547 * t2772 + 0.26020884564615598386e1_f64 * t25383 * t26441 + 0.43368140941025997312e-1_f64 * t95872 - 0.65854491829355115987e0_f64 * t7403 * t10978 + 0.21684070470512998656e-1_f64 * t95876 + 0.13010442282307799193e1_f64 * t7070 * t7076 * t7398 * t2645 * t231 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t2061 * t10638 * t231 + 0.51405703062096148812e-1_f64 * t95888 + t95891 - t95893 - 0.19756347548806534796e1_f64 * t95894 * t887 + t95899 - 0.77108554593144223218e-1_f64 * t95900 + 0.13709901006661042888e-1_f64 * t95902;
    t95904
}
