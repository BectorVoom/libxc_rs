//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 970/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk970(t2055: f64, t5517: f64, t72: f64, t8094: f64, t686: f64, t25878: f64, t25895: f64, t1882: f64, t543: f64, t7506: f64, t7301: f64, t27884: f64, t7515: f64) -> (f64, f64, f64, f64, f64) {
    let t28760 = t5517 * t2055;
    let t28779 = t8094 * t72;
    let t28780 = t28779 * t686;
    let t28781 = t25878 * t28780;
    let t28783 = t25895 * t28780;
    let t28791 = t7506 * t1882 * t543;
    let t28792 = t7301 * t28791;
    let t28796 = t27884 * t7515;
    (t28760, t28781, t28783, t28792, t28796)
}
