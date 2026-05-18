//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1015/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1015<F: Float>(t1: F, t1184: F, t2071: F, t548: F, t4036: F, t835: F, t11675: F, t11678: F, t11681: F, t11685: F, t11686: F, t11891: F, t11892: F, t11894: F, t11895: F, t11897: F) -> (F, F, F, F) {
    let t11898 = t1 * t1184;
    let t11900 = t548 * t11898 * t2071;
    let t11901 = F::new(64.0) / F::new(45.0) * t11900;
    let t11903 = F::new(4.0) / F::new(5.0) * t4036 * t835;
    let t11904 = t11675 + t11678 - t11681 - t11685 + t11686 - t11891 - t11892 - t11894 - t11895 - t11897 - t11901 - t11903;
    (t11898, t11901, t11903, t11904)
}
