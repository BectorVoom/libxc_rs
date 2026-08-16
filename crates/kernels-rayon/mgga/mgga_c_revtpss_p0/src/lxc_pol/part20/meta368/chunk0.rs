//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1342/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1342(t40135: f64, t760: f64, t10565: f64, t606: f64, t706: f64, t10468: f64, t750: f64, t10555: f64, t10605: f64, t10436: f64, t2398: f64, t10356: f64, t10439: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40137 = 0.6233709278045326953e3_f64 * t760 * t40135;
    let t40139 = t706 * t10565 * t606;
    let t40140 = 16.0_f64 * t40139;
    let t40141 = t10468 * t750;
    let t40142 = 4.0_f64 * t40141;
    let t40143 = t10605 * t10555;
    let t40144 = 144.0_f64 * t40143;
    let t40145 = t2398 * t10436;
    let t40146 = 48.0_f64 * t40145;
    let t40148 = t10439 * t750 * t10356;
    (t40137, t40140, t40142, t40144, t40146, t40148)
}
