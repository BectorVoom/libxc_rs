//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1299/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1299<F: Float>(t16732: F, t16736: F, t16740: F, t16742: F, t16746: F, t16750: F, t16752: F, t16755: F, t16758: F, t16760: F, t16763: F, t16766: F, t16769: F, t16772: F, t16775: F, t16777: F, t16779: F) -> (F,) {
    let t19192 = t16732 + t16736 + t16740 - t16742 - t16746 + t16750 + t16752 + t16755 + t16758 + t16760 + t16763 + t16766 - t16769 + t16772 - t16775 - t16777 + t16779;
    (t19192,)
}
