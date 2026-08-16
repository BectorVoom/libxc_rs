//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 388/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk388(t1974: f64, t110: f64, t518: f64, t84: f64, t596: f64, t1847: f64, t1849: f64, t587: f64) -> (f64, f64, f64, f64) {
    let t1975 = 1.0_f64 / t1974;
    let t1983 = t518 * t110 * t84;
    let t1985 = 0.24415406715670879921e-3_f64 * t596 * t1983;
    let t1990 = t1847 * t1849 * t587;
    (t1975, t1983, t1985, t1990)
}
