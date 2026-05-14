//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1157/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1157<F: Float>(t53729: F, t51651: F, t51667: F, t51683: F, t51688: F, t52131: F, t52432: F, t53734: F, t53736: F, t53742: F, t53748: F, t53751: F, t53758: F, t53768: F, t53772: F, t8793: F) -> (F,) {
    let t55351 = 7.0 / 576.0 * t53729;
    let t55367 = -t55351 + t53734 / 24.0 - t53736 / 24.0 + t53742 / 768.0 - t53748 / 192.0 - 35.0 / 54.0 * t51651 + t53751 / 48.0 - 7.0 / 288.0 * t51667 + t53758 / 48.0 - 7.0 / 72.0 * t52432 + t8793 * t52131 / 48.0 - t53768 / 1536.0 - 7.0 / 24.0 * t51683 - 7.0 / 144.0 * t51688 - t53772 / 48.0;
    (t55367,)
}
