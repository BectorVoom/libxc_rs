//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1101/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1101<F: Float>(t11703: F, t3688: F, t10141: F, t10150: F, t10156: F, t10166: F, t10172: F, t1111: F, t1112: F, t1114: F, t1115: F, t1117: F, t1148: F, t11659: F, t11668: F, t11672: F, t11680: F, t11690: F, t11694: F, t1520: F, t3708: F, t3803: F, t4574: F, t4581: F, t4584: F, t4588: F, t4600: F, t4603: F, t4608: F, t505: F, t511: F, t529: F) -> (F, F) {
    let t11704 = t11703 * t3688;
    let t11707 = 12.0 * t1117 * t4588 * t1111 - 24.0 * t511 * t11659 * t1114 - t4608 * t1115 + 2.0 * t1112 * t4584 + 2.0 * t3803 * t4600 - 336.0 * t529 * t11668 * t1114 - 6.0 * t505 * t11672 - 8.0 * t3803 * t4574 - 4.0 * t1117 * t4603 * t1111 + 6.0 * t511 * t11680 * t1114 + 4.0 * t1520 * t3708 + 252.0 * t1148 * t4581 * t1111 - 0.13333333333333333333e0 * t10166 * t11690 + 0.2e0 * t10141 * t11694 - 1.0 * t10150 * t11690 + 0.12e1 * t10156 * t11694 - 0.12e1 * t10156 * t11690 + 0.1536e-5 * t10172 * t11704;
    (t11704, t11707)
}
