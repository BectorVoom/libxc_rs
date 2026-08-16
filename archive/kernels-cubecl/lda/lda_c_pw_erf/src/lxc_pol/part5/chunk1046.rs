//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1046/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1046<F: Float>(t169: F, t301: F, t6080: F, t717: F, t1184: F, t1187: F, t2363: F, t483: F, t684: F, t7067: F, t1738: F, t2375: F) -> (F, F, F, F) {
    let t18835 = t169 * t717 * t6080 * t301;
    let t18866 = t1184 * t2363 * t483 * t1187;
    let t18876 = t684 * t7067;
    let t18880 = t1738 * t2375;
    (t18835, t18866, t18876, t18880)
}
