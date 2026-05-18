//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1204/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1204<F: Float>(t12737: F, t12741: F, t12743: F, t12746: F, t12751: F, t12753: F, t12758: F, t12760: F, t12763: F, t12766: F, t12768: F, t12771: F, t12775: F, t12778: F, t12783: F, t12785: F, t12788: F, t12794: F, t12798: F, t12801: F, t12803: F, t12808: F, t12810: F) -> (F, F) {
    let t14388 = -t12737 - t12741 - t12743 - t12746 - t12751 + t12753 + t12758 - t12760 - t12763 - t12766 - t12768;
    let t14389 = -t12771 - t12775 - t12778 - t12783 - t12785 - t12788 + t12794 + t12798 - t12801 + t12803 - t12808 + t12810;
    (t14388, t14389)
}
