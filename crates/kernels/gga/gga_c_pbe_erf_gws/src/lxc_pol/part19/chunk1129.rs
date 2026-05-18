//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1129/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1129<F: Float>(t14423: F, t361: F, t3223: F, t13917: F, t1162: F, t875: F, t13796: F, t3989: F, t2171: F, t13859: F, t2409: F, t9721: F) -> (F, F, F, F, F, F, F) {
    let t14424 = t361 * t14423;
    let t14425 = t14424 * t3223;
    let t14426 = t13917 * t14425;
    let t14442 = t1162 * t875;
    let t14443 = t13796 * t14442;
    let t14444 = t3989 * t14443;
    let t14455 = t14423 * t2171;
    let t14456 = t13796 * t14455;
    let t14457 = t13859 * t14456;
    let t14463 = t2409 * t9721;
    (t14425, t14426, t14443, t14444, t14456, t14457, t14463)
}
