//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 864/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk864<F: Float>(t2801: F, t992: F, t2882: F, t2881: F, t14075: F, t4265: F, t4267: F, t8392: F, t2739: F, t2875: F, t2874: F, t2413: F, t4150: F, t10771: F, t10773: F, t10804: F, t11593: F, t15500: F, t15502: F, t15504: F, t15508: F, t15511: F, t15515: F, t15519: F, t1901: F, t446: F) -> (F,) {
    let t15522 = t992 * t2801;
    let t15523 = t2882 * t15522;
    let t15524 = t2881 * t15523;
    let t15527 = t4265 * t14075;
    let t15528 = t2881 * t15527;
    let t15532 = 4.0 / 27.0 * t8392 * t4267;
    let t15533 = t992 * t2739;
    let t15534 = t2875 * t15533;
    let t15535 = t2874 * t15534;
    let t15538 = t4150 * t2413;
    let t15539 = t2881 * t15538;
    let t15543 = t10771 / 9.0 - 8.0 / 27.0 * t10773 - t15500 - t15502 - t446 * t15504 / 9.0 - 2.0 / 27.0 * t446 * t15508 + 2.0 / 9.0 * t1901 * t15511 + 8.0 / 27.0 * t11593 * t15515 - 2.0 / 9.0 * t1901 * t15519 + t1901 * t15524 / 9.0 + 2.0 / 9.0 * t1901 * t15528 - t15532 + t1901 * t15535 / 9.0 + t1901 * t15539 / 9.0 - 2.0 / 9.0 * t10804;
    (t15543,)
}
