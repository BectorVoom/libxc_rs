//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 818/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk818(t10261: f64, t21978: f64, t27: f64, t89: f64, t4056: f64, t5299: f64, t193: f64, t20489: f64, t792: f64, t666: f64, t21181: f64, t2660: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21979 = t10261 * t21978;
    let t21981 = t89 * t27 * t21979;
    let t21982 = t4056 * t5299;
    let t21984 = t89 * t193 * t21982;
    let t21985 = t792 * t20489;
    let t21987 = t89 * t666 * t21985;
    let t21989 = t2660 * t21181;
    (t21979, t21981, t21982, t21984, t21985, t21987, t21989)
}
