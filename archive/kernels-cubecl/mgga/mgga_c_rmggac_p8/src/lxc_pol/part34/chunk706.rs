//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 706/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk706<F: Float>(t2084: F, t2123: F, t2145: F, t27: F, t14088: F, t21: F, t132: F, t14090: F, t240: F, t31: F, t4738: F, t71: F) -> (F, F) {
    let t69689 = t2145 * t27 * t2084 * t2123;
    let t69695 = t21 * t14088;
    let t69701 = t69695 * t14090 * t71 * t132 * t240 * t4738 * t31;
    (t69689, t69701)
}
