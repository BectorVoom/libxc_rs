//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1360/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1360<F: Float>(t110605: F, t1163: F, t33437: F, t32013: F, t3936: F, t33544: F, t3748: F, t12841: F, t33547: F, t113708: F, t9446: F, t13900: F, t9804: F, t21499: F, t33383: F, t110558: F, t113690: F, t113724: F, t20130: F, t32008: F, t32022: F, t32087: F, t32090: F, t32189: F, t32216: F, t33384: F, t33439: F, t6204: F, t81168: F) -> (F, F, F, F) {
    let t113909 = t110605 * t33437 * t1163;
    let t113914 = t3936 * t32013;
    let t113920 = t3748 * t33544;
    let t113922 = t12841 * t33547;
    let t113923 = 0.3684876543209876543e-2 * t113922;
    let t113933 = 0.69444444444444444446e-2 * t9446 * t113708;
    let t113939 = t9446 * t13900 * t9804;
    let t113941 = t33383 * t21499;
    let t113944 = -0.69444444444444444446e-2 * t32087 * t113909 - 0.26805555555555555556e-2 * t32008 * t113909 - 0.13888888888888888889e-1 * t32087 * t113914 * t20130 - 0.53611111111111111112e-2 * t32008 * t113724 - 0.44218518518518518517e-2 * t113920 + t113923 + 0.69444444444444444446e-2 * t33384 * t32216 + 0.62500000000000000002e-1 * t9446 * t6204 * t110558 * t81168 + 0.55555555555555555558e-1 * t32022 * t33439 - t113933 + 0.20833333333333333334e-1 * t9446 * t113690 + 0.21444444444444444446e-1 * t32189 * t33439 + 0.77160493827160493827e-3 * t113939 + 0.69444444444444444446e-2 * t113941 * t32090;
    (t113920, t113922, t113941, t113944)
}
