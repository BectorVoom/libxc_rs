//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 774/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk774<F: Float>(t5754: F, t732: F, t5483: F, t5496: F, t5502: F, t5583: F, t5587: F, t5736: F, t5740: F, t5744: F, t5751: F, t5753: F, t5519: F, t5522: F, t5525: F, t5539: F) -> (F, F, F, F) {
    let t5756 = 0.17544670867903938621e1 * t5754 * t732;
    let t5757 = -t5736 + t5740 - t5583 + t5587 - t5744 - t5751 - t5753 - t5756 - t5483 - t5496 + t5502;
    let t5758 = 0.28842592592592592592e-1 * t5519;
    let t5762 = -t5758 + 0.37083333333333333334e-1 * t5522 - 0.278125e-1 * t5525 + 0.278125e-1 * t5539;
    (t5756, t5757, t5758, t5762)
}
