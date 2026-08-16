//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 737/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk737(t3949: f64, t8676: f64, t8674: f64, t5462: f64, t8673: f64, t3954: f64, t154: f64, t125: f64, t1736: f64, t190: f64, t1649: f64, t1026: f64, t1754: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8677 = t8676 * t3949;
    let t8678 = t8674 * t8677;
    let t8680 = t5462 * t8673;
    let t8681 = t8676 * t3954;
    let t8682 = t8680 * t8681;
    let t8684 = t5462 * t154;
    let t8685 = t1736 * t125;
    let t8686 = t8685 * t190;
    let t8687 = t8686 * t1649;
    let t8688 = t8684 * t8687;
    let t8690 = t1754 * t1026;
    (t8677, t8678, t8681, t8682, t8684, t8685, t8686, t8687, t8688, t8690)
}
