//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1143/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1143(t3123: f64, t3168: f64, t3124: f64, t3173: f64, t11231: f64, t4806: f64, t1042: f64, t1065: f64, t675: f64, t247: f64, t906: f64, t1063: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11977 = t3123 * t3168;
    let t11980 = t3124 * t3173;
    let t11982 = t4806 * t11231;
    let t11983 = t1042 * t11982;
    let t11986 = t675 * t1065;
    let t11988 = t247 * t11986 * t906;
    let t11989 = t1063 * t11988;
    (t11977, t11980, t11982, t11983, t11986, t11988, t11989)
}
