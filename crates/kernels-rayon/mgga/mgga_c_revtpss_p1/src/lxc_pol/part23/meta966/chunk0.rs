//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3264/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3264(t48262: f64, t47011: f64, t48269: f64, t22789: f64, t72: f64, t757: f64, t73476: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t39807: f64, t39813: f64, t47059: f64, t48261: f64, t48266: f64, t48268: f64, t48271: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t85908 = 0.17544670867903938621e1_f64 * t48262;
    let t85909 = 0.56968947174242584612e-3_f64 * t47011;
    let t85910 = 0.15584273195113317383e3_f64 * t48269;
    let t85912 = t22789 * t72 * t757;
    let t85913 = 0.18311447306006545054e-3_f64 * t85912;
    let t85914 = 3.0_f64 * t73476;
    let t85915 = t48261 - t85908 - t39783 - t39786 - t39791 - t39795 - t85909 + t48266 + t48268 - t85910 - t85913 + t48271 + t39799 + t47059 + t85914 + t39807 - t39813;
    (t85908, t85909, t85910, t85913, t85914, t85915)
}
