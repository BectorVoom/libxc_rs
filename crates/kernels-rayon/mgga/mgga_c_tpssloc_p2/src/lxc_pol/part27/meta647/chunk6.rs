//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2236/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2236(t362: f64, t4657: f64, t1598: f64, t974: f64, t23631: f64, t1920: f64, t25535: f64, t968: f64, t1003: f64, t1049: f64, t1058: f64, t1060: f64, t11059: f64, t14577: f64, t23633: f64, t23658: f64, t25429: f64, t25510: f64, t25550: f64, t25553: f64, t25706: f64, t25723: f64, t2770: f64, t2771: f64, t2780: f64, t3120: f64, t3961: f64, t6687: f64, t6784: f64, t6800: f64, t7593: f64, t7619: f64, t82668: f64, t82714: f64, t82717: f64, t83239: f64, t88016: f64, t884: f64) -> f64 {
    let t89235 = t362 * t4657;
    let t89242 = t974 * t1598;
    let t89243 = t23631 * t89242;
    let t89256 = 0.54831135561607547884e-2_f64 * t1920 * t968 * t25535;
    let t89265 = 0.73108180748810063846e-2_f64 * t25429 * t25510 * t1049 * t2770 * t3961 - 0.14621636149762012769e-1_f64 * t82714 - 0.36554090374405031922e-2_f64 * t82717 - 0.19495514866349350359e-1_f64 * t88016 * t25723 + 0.54831135561607547884e-2_f64 * t6687 * t6784 * t89235 * t884 - 0.14621636149762012769e-1_f64 * t82668 * t25550 - 0.16449340668482264365e-1_f64 * t89243 * t23658 + 6.0_f64 * t11059 * t7619 * t14577 + t1058 * t7593 * t3120 * t1060 + 2.0_f64 * t1003 * t25706 + t89256 + 0.27415567780803773942e-2_f64 * t23633 * t25553 * t6800 * t2780 + 0.36554090374405031923e-2_f64 * t83239 * t25553 * t6800 * t2771;
    t89265
}
