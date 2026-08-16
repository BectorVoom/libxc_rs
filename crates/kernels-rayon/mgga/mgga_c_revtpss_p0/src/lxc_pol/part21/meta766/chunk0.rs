//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2716/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2716(t10428: f64, t4305: f64, t2609: f64, t4186: f64, t706: f64, t10436: f64, t4311: f64, t14426: f64, t72: f64, t757: f64, t18875: f64, t2403: f64, t2411: f64, t2832: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t39807: f64, t39813: f64) -> (f64, f64, f64, f64, f64) {
    let t49978 = t10428 * t4305;
    let t49979 = 12.0_f64 * t49978;
    let t49981 = t706 * t2609 * t4186;
    let t49982 = 12.0_f64 * t49981;
    let t49983 = t4311 * t10436;
    let t49984 = 12.0_f64 * t49983;
    let t49986 = t14426 * t72 * t757;
    let t49987 = 0.54934341918019635162e-3_f64 * t49986;
    let t49988 = -9.0_f64 * t18875 * t2403 * t2411 * t2832 - t39786 - t39791 - t39795 + t39799 + t39807 - t39813 + t49979 + t49982 + t49984 - t49987;
    (t49979, t49982, t49984, t49987, t49988)
}
