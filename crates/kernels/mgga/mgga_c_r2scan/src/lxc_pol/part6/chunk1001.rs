//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1001/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1001<F: Float>(t1810: F, t963: F, t2798: F, t584: F, t1759: F, t1748: F, t2788: F, t1416: F, t959: F, t5237: F, t7647: F, t7650: F, t7653: F, t7656: F, t7659: F, t7661: F, t7662: F) -> (F, F) {
    let t7664 = t963 * t1810;
    let t7666 = t584 * t2798;
    let t7667 = t7666 * t1759;
    let t7669 = t2788 * t1748;
    let t7671 = t1416 * t959;
    let t7673 = -0.21687162600603479684e-1 * t5237 + 0.19263893255070628431e1 * t7647 + 0.1714584e0 * t7650 - t7653 + t7656 + t7659 + t7661 - 0.10389515463408878255e3 * t7662 + 0.35089341735807877242e1 * t7664 - 0.33872559466666666666e-2 * t7667 + 0.72290542002011598948e-2 * t7669 - 20.0 * t7671;
    (t7666, t7673)
}
