//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 676/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk676<F: Float>(t12423: F, t6485: F, t882: F, t3158: F, t6470: F, t874: F, t9439: F, t9438: F, t587: F, t9448: F, t2487: F, t6590: F, t9291: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12424 = t6485 * t12423;
    let t12425 = t882 * t12424;
    let t12427 = t3158 * t6470;
    let t12428 = t882 * t12427;
    let t12444 = t9439 * t874;
    let t12445 = t9438 * t12444;
    let t12446 = t587 * t12445;
    let t12448 = t9448 * t874;
    let t12449 = t9438 * t12448;
    let t12450 = t2487 * t12449;
    let t12452 = t9291 * t6590;
    (t12424, t12425, t12427, t12428, t12444, t12445, t12446, t12448, t12449, t12450, t12452)
}
