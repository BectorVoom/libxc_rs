//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1328/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1328(t28273: f64, t6547: f64, t28264: f64, t225: f64, t28282: f64, t28272: f64, t6562: f64, t794: f64, t23164: f64, t7479: f64, t86893: f64, t2174: f64, t6470: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98995 = t6547 * t28273;
    let t99003 = t6547 * t28264;
    let t99010 = t28282 * t225;
    let t99022 = t6562 * t794 * t28272;
    let t99036 = t23164 * t86893 * t7479;
    let t103103 = t6470 * t2174;
    (t98995, t99003, t99010, t99022, t99036, t103103)
}
