//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1191/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1191(t23573: f64, t24657: f64, t22015: f64, t894: f64, t23459: f64, t23465: f64, t23468: f64, t23788: f64, t23793: f64, t23807: f64, t23810: f64, t23815: f64, t23821: f64, t23946: f64, t24017: f64) -> (f64, f64) {
    let t24658 = t24657 * t23573;
    let t24660 = t894 * t24658 * t22015;
    let t24663 = t23459 - t23465 + t23468 + t23788 - t23793 - t23807 - t23810 - t23815 - t23821 - t23946 - t24017;
    (t24660, t24663)
}
