//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1282/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1282(t49498: f64, t49500: f64, t49507: f64, t49514: f64, t49521: f64, t49528: f64, t49538: f64, t49540: f64, t49545: f64, t49550: f64, t49555: f64, t49561: f64, t49567: f64, t49572: f64, t49576: f64, t49577: f64, t49579: f64, t49581: f64, t49585: f64, t49588: f64, t49594: f64, t49607: f64) -> (f64, f64) {
    let t50568 = -t49498 + t49500 - t49507 - t49514 + t49521 + t49528 + t49538 - t49540 - t49545 + t49550 + t49555;
    let t50571 = t49561 + t49567 + t49572 + t49576 - t49577 + t49579 + t49581 - t49585 - t49588 + t49594 - t49607;
    (t50568, t50571)
}
