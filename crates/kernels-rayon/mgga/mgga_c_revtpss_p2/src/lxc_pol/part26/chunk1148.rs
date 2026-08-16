//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1148/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1148(t7256: f64, t9784: f64, t25877: f64, t94390: f64, t1032: f64, t4066: f64, t1955: f64, t1399: f64, t2434: f64, t3924: f64, t676: f64, t10008: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94570 = t9784 * t7256;
    let t94589 = t94390 * t25877;
    let t94609 = t4066 * t1032;
    let t94610 = t1955 * t94609;
    let t94633 = t2434 * t1399;
    let t94639 = t676 * t3924;
    let t94643 = t1955 * t10008;
    (t94570, t94589, t94609, t94610, t94633, t94639, t94643)
}
