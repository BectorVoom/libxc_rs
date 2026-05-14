//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1236/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1236<F: Float>(t5272: F, t963: F, t1827: F, t2747: F, t1831: F, t406: F, t7705: F, t1419: F, t2755: F, t1416: F, t5435: F, t704: F, t898: F, t5431: F, t1745: F, t22387: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26732 = t963 * t5272;
    let t26736 = t2747 * t1827;
    let t26737 = 0.70178683471615754484e1 * t26736;
    let t26738 = t2747 * t1831;
    let t26739 = 0.35089341735807877242e1 * t26738;
    let t26741 = 24.0 * t406 * t7705;
    let t26742 = t1419 * t2755;
    let t26743 = 36.0 * t26742;
    let t26744 = t1416 * t2755;
    let t26745 = 60.0 * t26744;
    let t26750 = t898 * t704 * t5435;
    let t26752 = t963 * t5431;
    let t26754 = t2747 * t1745;
    let t26755 = 0.17544670867903938621e1 * t26754;
    let t26756 = 192.0 * t22387;
    (t26732, t26737, t26739, t26741, t26743, t26745, t26750, t26752, t26755, t26756)
}
