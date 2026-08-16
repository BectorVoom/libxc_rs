//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2213/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2213(t23479: f64, t25637: f64, t6722: f64, t1409: f64, t344: f64, t1009: f64, t6740: f64, t23473: f64, t13528: f64, t13542: f64, t13931: f64, t14130: f64, t1618: f64, t1920: f64, t1933: f64, t1934: f64, t1935: f64, t23414: f64, t23419: f64, t23495: f64, t25601: f64, t25609: f64, t2987: f64, t343: f64, t4509: f64, t4540: f64, t6730: f64, t6734: f64, t6735: f64, t7578: f64, t82880: f64, t83004: f64, t83025: f64, t83028: f64) -> f64 {
    let t88440 = 0.16149102437656156342e-2_f64 * t6722 * t25637 * t23479;
    let t88449 = t1409 * t344;
    let t88451 = t6740 * t88449 * t1009;
    let t88453 = 0.20186378047070195428e-3_f64 * t88451 * t23473;
    let t88472 = t88440 - t82880 * t1618 / 144.0_f64 - 0.20186378047070195428e-3_f64 * t1933 * t1934 * t4540 * t6735 - 0.10093189023535097714e-3_f64 * t25601 * t23495 + t88453 - t1920 * t2987 * t13542 / 72.0_f64 + t1920 * t4509 * t13528 / 108.0_f64 - 0.10093189023535097714e-3_f64 * t1935 * t13931 * t343 * t6734 - 0.10093189023535097714e-3_f64 * t23414 * t7578 - 0.20186378047070195428e-3_f64 * t6730 * t25609 + t83004 / 1728.0_f64 - t23419 * t14130 / 1152.0_f64 + t83025 / 81.0_f64 + t83028;
    t88472
}
