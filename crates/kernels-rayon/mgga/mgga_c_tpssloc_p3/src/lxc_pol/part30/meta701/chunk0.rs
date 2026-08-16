//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2265/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2265(t23164: f64, t7479: f64, t86893: f64, t17063: f64, t23278: f64, t25168: f64, t5637: f64, t82294: f64, t87748: f64, t87902: f64, t87911: f64, t87927: f64, t87932: f64, t92954: f64, t92961: f64, t99033: f64) -> f64 {
    let t99036 = t23164 * t86893 * t7479;
    let t99038 = -t92954 + t87902 + t87911 - t92961 - 0.49348022005446793095e-1_f64 * t87927 + 24.0_f64 * t25168 * t87748 * t17063 + 2.0_f64 * t23278 * t5637 - 0.52089578783527170488e-1_f64 * t82294 - 0.3289868133696452873e-1_f64 * t99033 + 0.16449340668482264365e-1_f64 * t99036 - t87932;
    t99038
}
