//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 972/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk972(t866: f64, t8800: f64, t36391: f64, t9222: f64, t35551: f64, t1679: f64, t7900: f64, t5016: f64, t8404: f64, t4601: f64, t8407: f64, t10820: f64, t1249: f64, t1356: f64, t1550: f64, t2368: f64, t2376: f64, t35795: f64, t35799: f64, t36305: f64, t36331: f64, t40597: f64, t40602: f64, t40607: f64, t40610: f64, t40614: f64) -> (f64, f64) {
    let t40616 = t8800 * t866;
    let t40619 = t9222 * t36391;
    let t40621 = t9222 * t35551;
    let t40623 = t1679 * t7900;
    let t40625 = t5016 * t8404;
    let t40627 = t4601 * t8407;
    let t40629 = 0.79828278012425390426e-1_f64 * t35795 - 0.11974241701863808564e0_f64 * t1356 * t40597 - 0.19957069503106347607e-1_f64 * t1249 * t2368 + t35799 - 0.11974241701863808564e0_f64 * t1550 * t40602 + 0.10227998120342003148e-1_f64 * t40607 - 0.4726e1_f64 * t36305 - 0.2993560425465952141e-1_f64 * t40610 - 0.11974241701863808564e0_f64 * t10820 * t2376 + 0.10227998120342003148e-1_f64 * t40614 + 0.39914139006212695214e-1_f64 * t1356 * t40616 - 0.31923449919973379548e-4_f64 * t40619 + 0.31923449919973379548e-4_f64 * t40621 - t36331 - 0.14635184302277988245e0_f64 * t40623 + 0.5987120850931904282e-1_f64 * t40625 - 0.8980681276397856423e-1_f64 * t40627;
    (t40616, t40629)
}
