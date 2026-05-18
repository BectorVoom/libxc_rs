//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1282/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1282<F: Float>(t49498: F, t49500: F, t49507: F, t49514: F, t49521: F, t49528: F, t49538: F, t49540: F, t49545: F, t49550: F, t49555: F, t49561: F, t49567: F, t49572: F, t49576: F, t49577: F, t49579: F, t49581: F, t49585: F, t49588: F, t49594: F, t49607: F) -> (F, F) {
    let t50568 = -t49498 + t49500 - t49507 - t49514 + t49521 + t49528 + t49538 - t49540 - t49545 + t49550 + t49555;
    let t50571 = t49561 + t49567 + t49572 + t49576 - t49577 + t49579 + t49581 - t49585 - t49588 + t49594 - t49607;
    (t50568, t50571)
}
