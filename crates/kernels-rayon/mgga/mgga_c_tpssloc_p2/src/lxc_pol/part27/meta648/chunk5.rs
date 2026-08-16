//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2243/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2243(t83244: f64, t974: f64, t985: f64, t3030: f64, t343: f64, t25483: f64, t25486: f64, t25490: f64, t25492: f64, t1022: f64, t1058: f64, t1060: f64, t23633: f64, t23670: f64, t23678: f64, t25479: f64, t25499: f64, t25554: f64, t25555: f64, t25705: f64, t25713: f64, t3200: f64, t4680: f64, t4684: f64, t6687: f64, t6743: f64, t82668: f64, t82823: f64, t82828: f64, t82830: f64, t83245: f64, t83246: f64, t88155: f64, t89375: f64) -> f64 {
    let t89498 = t83244 * t974 * t985;
    let t89499 = t343 * t3030;
    let t89501 = t89499 * t25483 * t25486;
    let t89505 = t89499 * t25490 * t25492;
    let t89515 = 0.43864908449286038306e-1_f64 * t23670 * t25479 + 0.54831135561607547884e-2_f64 * t23633 * t6743 * t4680 * t25554 + 2.0_f64 * t1058 * t25705 * t1022 * t1060 + 0.18277045187202515961e-2_f64 * t82823 + 0.54831135561607547884e-2_f64 * t83245 * t83246 * t89375 * t23678 - 0.82246703342411321825e-2_f64 * t6687 * t88155 * t25713 - 0.3289868133696452873e-1_f64 * t89498 * t89501 + 0.16449340668482264365e-1_f64 * t89498 * t89505 + 0.27415567780803773942e-2_f64 * t82828 + 0.97477574331746751793e-2_f64 * t82830 - 0.14621636149762012769e-1_f64 * t82668 * t25555 - 2.0_f64 * t3200 * t25499 * t4684;
    t89515
}
