//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1415/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1415<F: Float>(t10370: F, t1306: F, t2457: F, t27479: F, t27481: F, t27484: F, t27488: F, t27491: F, t27493: F, t27496: F, t27498: F, t27500: F, t27503: F, t27507: F, t27509: F, t27512: F, t27516: F, t27519: F, t27521: F, t27523: F, t27525: F, t27527: F, t27530: F, t27948: F, t27950: F, t27952: F, t27954: F, t27956: F, t3286: F, t8563: F) -> (F, F) {
    let t28589 = 2.0 * t10370 * t1306 * t2457 - t27479 + t27481 - t27484 + t27488 - t27491 + t27493 + t27496 + t27498 + t27500 - t27503 + t27507 + t27509;
    let t28593 = -2.0 * t1306 * t3286 * t8563 - t27512 - t27516 - t27519 + t27521 + t27523 - t27525 + t27527 + t27530 + t27948 - t27950 - t27952 - t27954 + t27956;
    (t28589, t28593)
}
