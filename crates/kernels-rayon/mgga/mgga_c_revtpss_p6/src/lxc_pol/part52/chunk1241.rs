//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1241/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1241(t32655: f64, t7983: f64, t28683: f64, t8692: f64, t121661: f64, t125336: f64, t125260: f64, t121656: f64, t125268: f64, t125279: f64, t32597: f64, t33621: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t128363 = t32655 * t7983;
    let t128367 = 2.0_f64 * t8692 * t28683;
    let t128368 = t121661 * t125336;
    let t128371 = t121661 * t125260;
    let t128374 = t121656 * t125268;
    let t128377 = t121656 * t125279;
    let t128382 = t32597 * t33621;
    (t128363, t128367, t128368, t128371, t128374, t128377, t128382)
}
