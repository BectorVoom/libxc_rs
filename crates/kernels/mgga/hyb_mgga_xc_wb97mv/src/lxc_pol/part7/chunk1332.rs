//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1332/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1332<F: Float>(t11623: F, t2694: F, t2685: F, t2704: F, t23839: F, t23843: F, t23847: F, t23851: F, t23854: F, t23857: F, t23863: F, t23990: F, t23992: F, t23994: F, t23999: F, t24003: F, t24008: F, t24015: F, t24016: F) -> (F,) {
    let t32542 = t11623 * t2694;
    let t32544 = t11623 * t2685;
    let t32546 = t11623 * t2704;
    let t32549 = t23839 + t23843 - t23847 - 0.11393789434848516922e-2 * t23990 + 0.70178683471615754484e1 * t23992 - 0.10389515463408878255e3 * t23994 + t23999 - t24003 - t24008 - t23851 - t23854 + t24015 - t23857 - 0.17315859105681463759e2 * t32542 - 0.5848223622634646207e0 * t32544 + 0.11696447245269292414e1 * t32546 - 24.0 * t24016 - t23863;
    (t32549,)
}
