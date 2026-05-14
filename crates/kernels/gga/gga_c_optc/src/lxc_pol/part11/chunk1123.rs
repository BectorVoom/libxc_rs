//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1123/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1123<F: Float>(t106: F, t1299: F, t13300: F, t13307: F, t13316: F, t16456: F, t16460: F, t16568: F, t167: F, t2106: F, t22933: F, t28700: F, t3454: F, t3461: F, t3462: F, t38463: F, t4675: F, t4723: F, t48629: F, t56122: F, t56187: F, t56227: F, t56350: F, t56373: F, t56380: F, t56422: F, t56457: F, t56489: F, t56521: F, t56553: F, t56587: F, t56610: F, t56631: F, t670: F) -> (F,) {
    let t56638 = 0.27818116767324025134e1 * t106 * (t56122 + t56187 + t56227 + t56350) * t167 - 0.11127246706929610054e2 * t106 * t48629 * t1299 + 0.33381740120788830161e2 * t106 * t38463 * t4675 - 0.1669087006039441508e2 * t106 * t13300 * t4723 - 0.66763480241577660323e2 * t106 * t28700 * t16456 + 0.66763480241577660323e2 * t13307 * t16460 - 0.11127246706929610054e2 * t106 * t3454 * t16568 + 0.6676348024157766032e2 * t106 * t22933 * t56373 - 0.10014522036236649048e3 * t3461 * t13316 * t4723 + 0.16690870060394415081e2 * t106 * t2106 * t56380 + 0.22254493413859220108e2 * t3461 * t3462 * t16568 - 0.27818116767324025134e1 * t106 * t670 * (t56422 + t56457 + t56489 + t56521 + t56553 + t56587 + t56610 + t56631);
    (t56638,)
}
