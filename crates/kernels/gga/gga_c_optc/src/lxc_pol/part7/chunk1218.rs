//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1218/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1218<F: Float>(t241: F, t26582: F, t26686: F, t26721: F, t26850: F, t3067: F, t8701: F, t26792: F, t26476: F, t26479: F, t26482: F, t26484: F, t26488: F, t26490: F, t26493: F, t26560: F, t26849: F) -> (F, F, F, F) {
    let t26853 = t241 * (t26582 + t26686 + t26721 + t26850);
    let t26855 = 0.41015588084031179722e4 * t3067 * t8701;
    let t26857 = 0.19751789702565206229e-1 * t241 * t26792;
    let t26858 = t26476 - t26479 - t26482 + t26484 + t26488 + t26490 + t26493 - t26560 + t26853 - t26855 + t26857 - t26849;
    (t26853, t26855, t26857, t26858)
}
