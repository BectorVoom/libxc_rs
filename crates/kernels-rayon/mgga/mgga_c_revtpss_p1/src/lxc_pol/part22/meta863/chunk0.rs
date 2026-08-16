//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3015/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3015(t10811: f64, t14707: f64, t14874: f64, t14673: f64, t40731: f64, t40593: f64, t4447: f64, t4462: f64, t10760: f64, t40763: f64, t4353: f64, t1559: f64, t775: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t50600 = t10811 * t14707;
    let t50602 = t10811 * t14874;
    let t50604 = t40731 * t14673;
    let t50606 = t40593 * t4447;
    let t50608 = t40593 * t4462;
    let t50611 = t10760 * t40763 * t4353;
    let t50613 = t1559 * t775;
    (t50600, t50602, t50604, t50606, t50608, t50611, t50613)
}
