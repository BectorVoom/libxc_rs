//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 848/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk848<F: Float>(t1862: F, t7657: F, t2743: F, t5322: F, t1838: F, t963: F, t1810: F, t2798: F, t584: F, t1759: F, t1748: F, t2788: F) -> (F, F, F, F, F, F) {
    let t7659 = F::new(0.2701041328e0) * t7657 * t1862;
    let t7661 = F::new(0.2701041328e0) * t2743 * t5322;
    let t7662 = t963 * t1838;
    let t7664 = t963 * t1810;
    let t7666 = t584 * t2798;
    let t7667 = t7666 * t1759;
    let t7669 = t2788 * t1748;
    (t7659, t7661, t7662, t7664, t7667, t7669)
}
