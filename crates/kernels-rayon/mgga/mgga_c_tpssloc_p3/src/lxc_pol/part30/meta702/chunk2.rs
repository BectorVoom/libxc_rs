//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2275/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2275(t28557: f64, t381: f64, t3173: f64, t5919: f64, t1921: f64, t28702: f64, t82431: f64, t1052: f64, t1409: f64, t1626: f64, t1634: f64, t17686: f64, t23327: f64, t23329: f64, t23330: f64, t23336: f64, t23369: f64, t254: f64, t25429: f64, t25731: f64, t25759: f64, t28475: f64, t28499: f64, t28713: f64, t3169: f64, t3174: f64, t3966: f64, t4693: f64, t5944: f64, t6680: f64, t6687: f64, t6691: f64, t88035: f64, t88758: f64, t986: f64) -> (f64, f64) {
    let t99273 = t28557 * t381;
    let t99296 = t3173 * t5919;
    let t99297 = t1921 * t99296;
    let t99301 = t82431 * t28702;
    let t99313 = -0.27415567780803773942e-2_f64 * t23327 * t99273 * t6691 - 0.54831135561607547884e-2_f64 * t23327 * t23329 * t23330 * t3966 * t1634 - 0.21932454224643019154e-1_f64 * t25429 * t23329 * t88035 * t17686 + 0.54831135561607547884e-2_f64 * t23327 * t23336 * t28499 - t23369 * t5944 - 0.21932454224643019153e-1_f64 * t6680 * t28475 + 4.0_f64 * t1052 * t3174 * t25731 * t1634 - 0.16449340668482264365e-1_f64 * t6687 * t986 * t99297 + t88758 - 0.18277045187202515961e-2_f64 * t99301 - 0.54831135561607547884e-2_f64 * t23327 * t23329 * t23330 * t1409 * t4693 - 12.0_f64 * t1626 * t254 * t25759 + 2.0_f64 * t3169 * t28713;
    (t99296, t99313)
}
