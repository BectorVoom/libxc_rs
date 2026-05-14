//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 859/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk859<F: Float>(t35551: F, t9222: F, t1679: F, t7900: F, t5016: F, t8404: F, t4601: F, t8407: F, t10820: F, t1249: F, t1356: F, t1550: F, t2368: F, t2376: F, t35795: F, t35799: F, t36305: F, t36331: F, t40597: F, t40602: F, t40607: F, t40610: F, t40614: F, t40616: F, t40619: F) -> (F,) {
    let t40621 = t9222 * t35551;
    let t40623 = t1679 * t7900;
    let t40625 = t5016 * t8404;
    let t40627 = t4601 * t8407;
    let t40629 = 0.79828278012425390426e-1 * t35795 - 0.11974241701863808564e0 * t1356 * t40597 - 0.19957069503106347607e-1 * t1249 * t2368 + t35799 - 0.11974241701863808564e0 * t1550 * t40602 + 0.10227998120342003148e-1 * t40607 - 0.4726e1 * t36305 - 0.2993560425465952141e-1 * t40610 - 0.11974241701863808564e0 * t10820 * t2376 + 0.10227998120342003148e-1 * t40614 + 0.39914139006212695214e-1 * t1356 * t40616 - 0.31923449919973379548e-4 * t40619 + 0.31923449919973379548e-4 * t40621 - t36331 - 0.14635184302277988245e0 * t40623 + 0.5987120850931904282e-1 * t40625 - 0.8980681276397856423e-1 * t40627;
    (t40629,)
}
