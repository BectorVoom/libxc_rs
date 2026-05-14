//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 904/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk904<F: Float>(t11900: F, t4036: F, t835: F, t11675: F, t11678: F, t11681: F, t11685: F, t11686: F, t11891: F, t11892: F, t11894: F, t11895: F, t11897: F, t1298: F, t4039: F, t3604: F, t5165: F) -> (F, F, F, F, F) {
    let t11901 = 64.0 / 45.0 * t11900;
    let t11903 = 4.0 / 5.0 * t4036 * t835;
    let t11904 = t11675 + t11678 - t11681 - t11685 + t11686 - t11891 - t11892 - t11894 - t11895 - t11897 - t11901 - t11903;
    let t11906 = 4.0 / 5.0 * t1298 * t4039;
    let t11907 = t5165 * t3604;
    (t11901, t11903, t11904, t11906, t11907)
}
