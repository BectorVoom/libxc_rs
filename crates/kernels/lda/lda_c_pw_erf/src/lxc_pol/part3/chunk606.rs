//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 606/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk606<F: Float>(t1321: F, t3854: F, t1318: F, t3796: F, t3801: F, t3805: F, t3810: F, t3814: F, t3816: F, t3821: F, t3823: F, t3827: F, t3831: F, t3836: F, t3840: F, t3843: F, t3845: F, t3849: F, t3853: F) -> (F, F, F, F) {
    let t3855 = t3854 * t1321;
    let t3856 = t1318 * t3855;
    let t3857 = 32.0 / 45.0 * t3856;
    let t3858 = -t3796 - t3801 - t3805 + t3810 + t3814 + t3816 + t3821 - t3823 - t3827 - t3831 - t3836 - t3840 - t3843 - t3845 - t3849 - t3853 + t3857;
    (t3855, t3856, t3857, t3858)
}
