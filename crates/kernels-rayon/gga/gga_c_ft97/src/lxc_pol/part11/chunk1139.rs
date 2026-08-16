//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1139/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1139(t43798: f64, t43841: f64, t43881: f64, t43922: f64, t845: f64, t91: f64, t27: f64, t43790: f64, t799: f64, t89: f64, t2999: f64, t825: f64) -> (f64, f64, f64) {
    let t43926 = t91 * t845 * (t43798 + t43841 + t43881 + t43922);
    let t43930 = t89 * t27 * t799 * t43790;
    let t43933 = t89 * t2999 * t825;
    (t43926, t43930, t43933)
}
