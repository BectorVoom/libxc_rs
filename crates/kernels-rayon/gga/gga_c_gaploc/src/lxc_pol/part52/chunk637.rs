//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 637/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk637(t11977: f64, t494: f64, t3689: f64, t599: f64, t475: f64, t2343: f64, t203: f64) -> (f64, f64, f64, f64, f64) {
    let t11978 = t11977 * t494;
    let t11981 = t599 * t3689;
    let t11982 = t11981 * t475;
    let t11983 = t2343 * t11982;
    let t11986 = t203 * t3689;
    (t11978, t11981, t11982, t11983, t11986)
}
