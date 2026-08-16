//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 471/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk471(t2221: f64, t4823: f64, t1053: f64, t920: f64, t2211: f64, t2210: f64, t167: f64, t4458: f64, t569: f64, t1901: f64, t2164: f64, t28: f64, t3460: f64, t3489: f64, t3545: f64, t3551: f64, t446: f64, t4726: f64, t4730: f64, t4735: f64, t4739: f64, t4743: f64, t4747: f64, t4792: f64, t4807: f64, t4811: f64, t4815: f64, t4819: f64, t89: f64) -> (f64, f64, f64, f64, f64) {
    let t4824 = t2221 * t4823;
    let t4827 = t920 * t1053;
    let t4828 = t2211 * t4827;
    let t4829 = t2210 * t4828;
    let t4833 = t569 * t167 * t4458;
    let t4837 = 2.0_f64 / 3.0_f64 * t446 * t4726 + 2.0_f64 / 3.0_f64 * t446 * t4730 + 2.0_f64 / 3.0_f64 * t446 * t4735 - 2.0_f64 / 9.0_f64 * t446 * t4739 - t446 * t4743 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t4747 + t2164 - 2.0_f64 / 9.0_f64 * t3489 + 2.0_f64 / 9.0_f64 * t3551 + 2.0_f64 / 9.0_f64 * t3545 + t89 * t28 * t4792 / 3.0_f64 - t446 * t4807 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t4811 - 2.0_f64 / 3.0_f64 * t446 * t4815 - t446 * t4819 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t4824 + 2.0_f64 / 9.0_f64 * t1901 * t4829 + 2.0_f64 / 9.0_f64 * t446 * t4833 + 2.0_f64 / 27.0_f64 * t3460;
    (t4824, t4828, t4829, t4833, t4837)
}
