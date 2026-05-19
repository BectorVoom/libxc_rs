//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 675/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk675<F: Float>(t638: F, t639: F, t9754: F, t1916: F, t511: F, t650: F, t1734: F, t645: F) -> (F, F, F, F) {
    let t9756 = t638 * t639 * t9754;
    let t9757 = F::cast_from(0.15243824895787514157e-3_f64) * t9756;
    let t9762 = t1916 * t511;
    let t9763 = t9762 * t650;
    let t9764 = F::cast_from(0.34093327067806677161e-2_f64) * t9763;
    let t9765 = t645 * t1734;
    (t9757, t9762, t9764, t9765)
}
