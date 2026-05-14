//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1140/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1140<F: Float>(t10027: F, t6725: F, t12414: F, t6730: F, t6734: F, t6737: F, t16740: F, t16742: F, t16746: F, t16750: F, t16752: F, t16755: F, t16758: F, t16760: F, t16763: F, t16766: F, t16769: F, t16772: F, t16775: F) -> (F, F, F, F, F) {
    let t16777 = 32.0 / 45.0 * t10027 * t6725;
    let t16779 = 32.0 / 45.0 * t12414 * t6730;
    let t16781 = 32.0 / 45.0 * t12414 * t6734;
    let t16783 = 16.0 / 27.0 * t12414 * t6737;
    let t16784 = t16740 - t16742 - t16746 + t16750 + t16752 + t16755 + t16758 + t16760 + t16763 + t16766 - t16769 + t16772 - t16775 - t16777 + t16779 + t16781 - t16783;
    (t16777, t16779, t16781, t16783, t16784)
}
