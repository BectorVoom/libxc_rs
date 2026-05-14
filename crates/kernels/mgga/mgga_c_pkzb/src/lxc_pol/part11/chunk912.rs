//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 912/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk912<F: Float>(t10639: F, t10674: F, t158: F, t1054: F, t3466: F, t5418: F, t2678: F, t3487: F, t10627: F, t183: F, t1044: F, t3410: F, t1034: F, t10573: F, t10577: F, t10621: F, t164: F, t167: F, t1717: F, t1721: F, t2682: F, t2693: F, t3441: F, t3460: F, t5389: F, t5391: F, t588: F) -> (F, F, F, F, F, F) {
    let t10675 = t10639 + t10674;
    let t10676 = t10675 * t158;
    let t10685 = t3466 * t1054;
    let t10686 = t5418 * t10685;
    let t10689 = t2678 * t3487;
    let t10692 = t183 * t10627;
    let t10696 = t1044 * t3410;
    let t10727 = -0.39512695097613069591e1 * t5389 * t10692 * t5391 + 0.39512695097613069591e1 * t1717 * t10696 * t1721 + 0.39512695097613069591e1 * t2682 * t10577 + 0.39512695097613069591e1 * t1717 * t10692 * t1721 - 0.19756347548806534796e1 * t588 * t3460 * t1034 * t164 - 0.19756347548806534796e1 * t588 * t1044 * t3441 * t164 - 0.19756347548806534796e1 * t588 * t10696 * t164 - 0.65854491829355115987e0 * t588 * t183 * t10621 * t164 - 0.19756347548806534796e1 * t2693 * t10573 - 0.65854491829355115987e0 * t588 * t10692 * t164 + 0.65854491829355115987e0 * t167 * t10675;
    (t10675, t10676, t10685, t10686, t10689, t10727)
}
