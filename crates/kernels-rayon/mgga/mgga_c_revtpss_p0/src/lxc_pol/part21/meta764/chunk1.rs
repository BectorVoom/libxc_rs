//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2713/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2713(t1522: f64, t40158: f64, t14362: f64, t9575: f64, t123: f64, t2630: f64, t4392: f64, t4398: f64, t9318: f64, t11231: f64, t14330: f64, t4402: f64) -> (f64, f64, f64, f64, f64) {
    let t49925 = 4.0_f64 * t40158 * t1522;
    let t49926 = t14362 * t9575;
    let t49927 = 0.21687162600603479684e-1_f64 * t49926;
    let t49929 = t4392 * t123 * t2630;
    let t49930 = 0.32530743900905219526e-1_f64 * t49929;
    let t49940 = t4398 * t9318;
    let t49941 = 0.35089341735807877242e1_f64 * t49940;
    let t49944 = 72.0_f64 * t14330 * t4402 * t11231;
    (t49925, t49927, t49930, t49941, t49944)
}
