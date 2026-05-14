//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1099/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1099<F: Float>(t13812: F, t2503: F, t13817: F, t1177: F, t1178: F, t371: F, t9689: F, t14617: F, t51581: F, t14135: F, t3039: F, t14138: F, t1112: F, t361: F, t51020: F, t874: F, t938: F) -> (F, F, F, F, F, F, F) {
    let t53751 = t13812 * t2503;
    let t53758 = t13817 * t2503;
    let t53768 = t1177 * t371 * t1178 * t9689;
    let t53772 = t51581 * t14617;
    let t53774 = t3039 * t14135;
    let t53775 = t53774 * t14138;
    let t53799 = t361 * t51020 * t1112;
    let t53800 = t938 * t874;
    (t53751, t53758, t53768, t53772, t53775, t53799, t53800)
}
