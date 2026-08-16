//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2594/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2594(t2246: f64, t5812: f64, t1469: f64, t627: f64, t72: f64, t10605: f64, t18539: f64, t11064: f64, t6075: f64, t37: f64, t5940: f64, t2609: f64, t5825: f64, t706: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60673 = t5812 * t2246;
    let t60823 = t1469 * t627 * t72;
    let t61020 = t10605 * t18539;
    let t61033 = t6075 * t11064;
    let t61037 = t37 * t5940;
    let t61090 = t706 * t2609 * t5825;
    (t60673, t60823, t61020, t61033, t61037, t61090)
}
