//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 763/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk763(t1580: f64, t920: f64, t3194: f64, t3193: f64, t100: f64, t8275: f64, t103: f64, t7763: f64, t11437: f64, t1651: f64, t3199: f64, t1902: f64) -> (f64, f64, f64, f64) {
    let t11982 = t920 * t1580;
    let t11983 = t3194 * t11982;
    let t11984 = t3193 * t11983;
    let t11987 = t8275 * t100;
    let t11988 = t103 * t7763;
    let t11989 = t11988 * t11437;
    let t11990 = t11987 * t11989;
    let t11993 = t3199 * t1651;
    let t11994 = t1902 * t11993;
    (t11982, t11984, t11990, t11994)
}
