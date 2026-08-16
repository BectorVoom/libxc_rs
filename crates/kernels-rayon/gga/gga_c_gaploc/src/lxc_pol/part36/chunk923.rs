//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 923/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk923(t209: f64, t42577: f64, t42621: f64, t42680: f64, t42723: f64, t42776: f64, t42818: f64, t42861: f64, t42900: f64, t29650: f64, t2972: f64, t13235: f64, t14537: f64) -> (f64, f64, f64) {
    let t42904 = (t42577 + t42621 + t42680 + t42723 + t42776 + t42818 + t42861 + t42900) * t209;
    let t42906 = 2.0_f64 * t29650 * t2972;
    let t42908 = 6.0_f64 * t14537 * t13235;
    (t42904, t42906, t42908)
}
