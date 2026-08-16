//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1086/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1086(t1469: f64, t8442: f64, t33624: f64, t644: f64, t8621: f64, t1497: f64, t36: f64, t606: f64, t125209: f64, t34258: f64, t7002: f64, t32392: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125314 = t8442 * t1469;
    let t125328 = t8621 * t33624 * t644;
    let t125335 = t1497 * t36;
    let t125336 = t125335 * t606;
    let t125337 = t8442 * t125336;
    let t125344 = 2.0_f64 * t125209;
    let t125377 = 4.0_f64 * t34258 * t7002;
    let t125379 = 4.0_f64 * t32392 * t7741;
    (t125314, t125328, t125337, t125344, t125377, t125379)
}
