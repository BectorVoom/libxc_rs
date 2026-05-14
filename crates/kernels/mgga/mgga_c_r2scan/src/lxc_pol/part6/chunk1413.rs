//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1413/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1413<F: Float>(t1823: F, t2747: F, t5261: F, t963: F, t5272: F, t741: F, t7803: F, t1827: F, t1831: F, t406: F, t7705: F, t1419: F, t2755: F, t1416: F, t21829: F, t21832: F, t22375: F, t22379: F) -> (F,) {
    let t26728 = t2747 * t1823;
    let t26729 = 0.30762056574649219973e4 * t26728;
    let t26730 = t963 * t5261;
    let t26732 = t963 * t5272;
    let t26734 = t7803 * t741;
    let t26736 = t2747 * t1827;
    let t26737 = 0.70178683471615754484e1 * t26736;
    let t26738 = t2747 * t1831;
    let t26739 = 0.35089341735807877242e1 * t26738;
    let t26741 = 24.0 * t406 * t7705;
    let t26742 = t1419 * t2755;
    let t26743 = 36.0 * t26742;
    let t26744 = t1416 * t2755;
    let t26745 = 60.0 * t26744;
    let t26746 = t26729 + 0.30762056574649219973e4 * t26730 + 0.91082604192152556044e5 * t26732 - 0.35089341735807877242e1 * t26734 - t26737 - t26739 - t26741 + t21829 + t21832 - t26743 + t22375 + t22379 - t26745;
    (t26746,)
}
