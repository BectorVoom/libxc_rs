//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1328/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1328<F: Float>(t11919: F, t4049: F, t13953: F, t15314: F, t11403: F, t3959: F, t11398: F, t11757: F, t3972: F, t3975: F, t11588: F, t14617: F, t53688: F) -> (F, F, F, F, F, F, F) {
    let t57235 = t4049 * t11919;
    let t57260 = t13953 * t15314;
    let t57262 = t3959 * t11403;
    let t57265 = t3959 * t11398;
    let t57284 = t3972 * t3975 * t11757;
    let t57287 = t3972 * t3975 * t11588;
    let t57289 = t53688 * t14617;
    (t57235, t57260, t57262, t57265, t57284, t57287, t57289)
}
