//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2737/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2737(t127: f64, t17693: f64, t20944: f64, t20946: f64, t1285: f64, t57659: f64, t17350: f64, t17934: f64, t17445: f64, t5373: f64, t12866: f64, t20933: f64, t56756: f64) -> (f64, f64, f64, f64, f64) {
    let t71435 = t17693 * t127 * t20944 * t20946;
    let t71440 = t1285 * t57659;
    let t71447 = t17934 * t17350;
    let t71460 = t5373 * t17445;
    let t71470 = t12866 * t56756 * t20933;
    (t71435, t71440, t71447, t71460, t71470)
}
