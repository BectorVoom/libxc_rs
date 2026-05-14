//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1204/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1204<F: Float>(t13796: F, t3989: F, t57321: F, t875: F, t14724: F, t3306: F, t343: F, t12206: F, t3965: F, t1193: F, t353: F, t3703: F, t8599: F, t14583: F, t50998: F, t53860: F) -> (F, F, F, F, F) {
    let t57740 = t3989 * t13796 * t57321 * t875;
    let t57745 = t3989 * t13796 * t14724 * t343 * t3306;
    let t57747 = t3965 * t12206;
    let t57751 = t8599 * t353 * t1193 * t3703;
    let t57755 = t50998 * t53860 * t14583;
    (t57740, t57745, t57747, t57751, t57755)
}
