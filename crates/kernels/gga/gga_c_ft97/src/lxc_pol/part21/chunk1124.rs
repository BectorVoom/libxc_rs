//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1124/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1124<F: Float>(t29483: F, t92370: F, t15772: F, t2258: F, t5579: F, t22767: F, t29558: F, t22632: F, t5611: F, t373: F, t384: F, t4491: F, t29550: F, t100554: F, t115337: F, t115341: F, t22522: F, t22565: F, t22568: F, t22603: F, t29469: F, t29474: F, t29498: F, t29515: F, t4474: F, t53: F, t5538: F, t5540: F, t5591: F, t72: F, t7839: F, t92303: F, t93229: F) -> (F, F, F, F, F, F, F) {
    let t115478 = t29483 * t92370;
    let t115486 = t5579 * t2258 * t15772;
    let t115492 = t22767 * t29558;
    let t115495 = t22632 * t29558;
    let t115496 = t5611 * t115495;
    let t115506 = t4491 * t373 * t384;
    let t115513 = t22767 * t29550;
    let t115516 = t22632 * t29550;
    let t115517 = t5611 * t115516;
    let t115525 = -0.17263005832038132092e-5 * t115478 - 0.27246626553445399074e-2 * t93229 * t5591 * t72 * t4474 * t53 - 0.6384360837962962963e-2 * t5611 * t115486 + 0.91830411319857336049e-5 * t92303 * t29469 * t7839 - 0.34049924469135802469e-1 * t5611 * t115492 + 0.42562405586419753087e-2 * t115496 + 0.49489226297715094073e-4 * t100554 - 0.68099848938271604939e-1 * t22522 * t22568 * t29498 - 0.13784064983740990796e-3 * t22565 * t29515 * t7839 - 0.25845121844514357744e-4 * t22603 * t5540 * t115506 + 0.27568129967481981593e-4 * t22565 * t29474 * t7839 + 0.17024962234567901235e-1 * t5611 * t115513 - 0.21281202793209876543e-2 * t115517 + 0.62028292426834458586e-5 * t5538 * t5540 * t115337 - 0.10338048737805743098e-4 * t5538 * t5540 * t115341;
    (t115486, t115492, t115495, t115506, t115513, t115516, t115525)
}
