//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 742/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk742(t1699: f64, t3336: f64, t1100: f64, t1102: f64, t198: f64, t336: f64, t4589: f64, t4592: f64, t4594: f64, t4597: f64, t4634: f64, t4638: f64, t4716: f64, t4718: f64, t4721: f64, t4723: f64, t4727: f64, t4731: f64, t4736: f64, t5019: f64, t5023: f64) -> (f64, f64) {
    let t5024 = t1699 * t3336;
    let t5027 = t1102 * t198 * t336 * t5019 - t1100 * t5023 * t5024 - t4589 + t4592 + t4594 - t4597 + t4634 + t4638 + t4716 + t4718 - t4721 - t4723 + t4727 - t4731 - t4736;
    (t5024, t5027)
}
