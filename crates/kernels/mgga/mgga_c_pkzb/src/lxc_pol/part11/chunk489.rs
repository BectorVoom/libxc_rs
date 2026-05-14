//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 489/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk489<F: Float>(t2363: F, t410: F, t2126: F, t2370: F, t914: F, t937: F, t2393: F, t394: F, t418: F) -> (F, F, F, F, F, F, F) {
    let t2433 = t2363 * t410;
    let t2435 = t2126 * t2370;
    let t2439 = t914 * t937;
    let t2446 = t2393 * t410;
    let t2447 = t2126 * t394;
    let t2463 = t418 * t418;
    let t2464 = 1.0 / t2463;
    (t2433, t2435, t2439, t2446, t2447, t2463, t2464)
}
