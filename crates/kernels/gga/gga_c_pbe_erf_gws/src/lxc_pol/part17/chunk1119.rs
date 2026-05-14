//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1119/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1119<F: Float>(t14135: F, t3039: F, t14138: F, t20154: F, t3067: F, t4155: F, t938: F, t2376: F, t26617: F, t810: F, t2408: F, t3212: F, t51084: F, t51540: F, t51667: F, t51683: F, t51688: F, t53748: F, t53750: F, t53751: F, t53758: F, t53761: F, t53768: F, t53772: F, t6793: F, t8629: F, t9283: F) -> (F,) {
    let t53774 = t3039 * t14135;
    let t53775 = t53774 * t14138;
    let t53779 = t20154 * t3067 * t4155 * t938;
    let t53784 = t26617 * t2376 * t4155 * t810;
    let t53787 = -t53748 / 384.0 - t53750 + t53751 / 96.0 - t2408 * t9283 * t51084 * t3212 / 12.0 - 7.0 / 576.0 * t51667 + t53758 / 96.0 + t6793 * t53761 / 24.0 + t8629 * t51540 / 48.0 - t53768 / 3072.0 - 7.0 / 48.0 * t51683 - 7.0 / 288.0 * t51688 - t53772 / 96.0 - t53775 / 48.0 - t6793 * t53779 / 12.0 - t6793 * t53784 / 8.0;
    (t53787,)
}
