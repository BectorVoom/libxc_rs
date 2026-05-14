//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 799/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk799<F: Float>(t7645: F, t7673: F, t7681: F, t7696: F, t7715: F, t7731: F, t7744: F, t7764: F, t7782: F, t7792: F, t7800: F, t7814: F, t7822: F, t7834: F, t7842: F, t7850: F) -> (F,) {
    let t7854 = t7645 + t7673 + t7681 + t7696 + t7715 + t7731 + t7744 + t7764 + t7782 + t7792 + t7800 + t7814 + t7822 + t7834 + t7842 + t7850;
    (t7854,)
}
