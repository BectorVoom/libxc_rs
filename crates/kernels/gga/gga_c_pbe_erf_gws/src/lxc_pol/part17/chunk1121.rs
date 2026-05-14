//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1121/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1121<F: Float>(t14121: F, t8806: F, t13917: F, t14424: F, t9371: F, t51898: F, t9243: F, t1105: F, t12213: F, t13994: F, t14106: F, t14627: F, t2376: F, t2408: F, t2409: F, t3066: F, t4385: F, t51719: F, t51724: F, t51726: F, t51745: F, t53790: F, t53795: F, t53804: F, t53807: F, t53809: F, t6781: F, t6793: F) -> (F,) {
    let t53811 = t14121 * t8806;
    let t53816 = t13917 * t14424 * t9371;
    let t53832 = t51898 * t9243;
    let t53834 = -t4385 * t53790 / 48.0 - t6793 * t53795 / 8.0 + t53804 / 768.0 - t53807 + 7.0 / 288.0 * t51719 + t53809 / 16.0 + t53811 / 8.0 - 7.0 / 144.0 * t51724 - 7.0 / 72.0 * t51726 - t53816 / 768.0 + t3066 * t2409 * t12213 * t13994 / 24.0 + t2408 * t2409 * t6781 * t14627 / 24.0 - 7.0 / 72.0 * t51745 + t2408 * t2409 * t2376 * t14106 * t1105 / 48.0 - t53832 / 4.0;
    (t53834,)
}
