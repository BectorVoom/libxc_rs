//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1201/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1201<F: Float>(t193: F, t202: F, t2522: F, t39529: F, t40760: F, t40762: F, t40764: F, t40766: F, t40768: F, t40769: F, t40772: F, t40777: F, t40779: F, t40782: F, t40784: F, t40785: F, t40790: F, t776: F) -> F {
    let t40791 = -F::cast_from(6.0_f64) * t193 * t202 * t40769 * t40772 + F::cast_from(24.0_f64) * t2522 * t40785 * t776 - t39529 + t40760 - t40762 + t40764 + t40766 + t40768 + t40777 - t40779 + t40782 + t40784 + t40790;
    t40791
}
