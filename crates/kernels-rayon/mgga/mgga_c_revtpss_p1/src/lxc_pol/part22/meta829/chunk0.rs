//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2948/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2948(t1868: f64, t9940: f64, t5577: f64, t588: f64, t5585: f64, t4010: f64, t5591: f64, t13921: f64, t221: f64, t4018: f64, t4019: f64, t2661: f64, t3924: f64, t3992: f64, t5651: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48347 = t9940 * t1868;
    let t48394 = 16.0_f64 * t5577 * t588;
    let t48417 = 16.0_f64 * t5585 * t588;
    let t48432 = t4010 * t5591;
    let t48445 = t4018 * t4019 * t221 * t13921;
    let t48449 = t2661 * t3992 * t5651 * t3924;
    (t48347, t48394, t48417, t48432, t48445, t48449)
}
