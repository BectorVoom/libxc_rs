//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1393/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1393<F: Float>(t76768: F, t77498: F, t77539: F, t77587: F, t77637: F, t77687: F, t77724: F, t77761: F, t1625: F, t21390: F, t5872: F, t6739: F) -> (F, F, F) {
    let t77764 = t76768 + t77498 + t77539 + t77587 + t77637 + t77687 + t77724 + t77761;
    let t77782 = t1625 * t21390;
    let t77794 = t6739 * t5872;
    (t77764, t77782, t77794)
}
