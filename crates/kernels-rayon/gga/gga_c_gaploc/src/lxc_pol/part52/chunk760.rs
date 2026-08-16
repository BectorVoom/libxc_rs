//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 760/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk760(t2021: f64, t36515: f64, t11756: f64, t783: f64, t321: f64, t3614: f64, t107: f64, t787: f64, t11844: f64, t1980: f64, t11848: f64, t35445: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36516 = t2021 * t36515;
    let t36590 = t11756 * t783;
    let t36610 = t321 * t3614;
    let t36612 = t787 * t36610 * t107;
    let t36632 = t1980 * t11844;
    let t36635 = t1980 * t11848;
    let t36654 = t739 * t35445;
    (t36516, t36590, t36610, t36612, t36632, t36635, t36654)
}
