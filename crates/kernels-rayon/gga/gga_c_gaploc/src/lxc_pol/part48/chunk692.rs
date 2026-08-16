//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 692/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk692(t13302: f64, t13331: f64, t209: f64, t11288: f64, t921: f64, t1016: f64, t10283: f64, t3366: f64, t8045: f64, t2798: f64, t3418: f64, t3553: f64, t6556: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13332 = t13302 + t13331;
    let t13333 = t13332 * t209;
    let t13334 = t11288 * t921;
    let t13336 = 2.0_f64 * t10283 * t1016;
    let t13338 = 4.0_f64 * t8045 * t3366;
    let t13340 = 2.0_f64 * t2798 * t3418;
    let t13342 = 2.0_f64 * t6556 * t3553;
    (t13332, t13333, t13334, t13336, t13338, t13340, t13342)
}
