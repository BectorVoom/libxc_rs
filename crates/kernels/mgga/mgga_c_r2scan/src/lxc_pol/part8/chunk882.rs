//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 882/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk882<F: Float>(t1861: F, t2768: F, t1860: F, t1859: F, t2482: F, t1862: F, t2743: F, t5322: F, t1838: F, t963: F, t1810: F, t2798: F, t584: F, t1759: F, t1748: F, t2788: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7654 = t2768 * t1861;
    let t7656 = 0.2701041328e0 * t1860 * t7654;
    let t7657 = t1859 * t2482;
    let t7659 = 0.2701041328e0 * t7657 * t1862;
    let t7661 = 0.2701041328e0 * t2743 * t5322;
    let t7662 = t963 * t1838;
    let t7664 = t963 * t1810;
    let t7666 = t584 * t2798;
    let t7667 = t7666 * t1759;
    let t7669 = t2788 * t1748;
    (t7654, t7656, t7657, t7659, t7661, t7662, t7664, t7666, t7667, t7669)
}
