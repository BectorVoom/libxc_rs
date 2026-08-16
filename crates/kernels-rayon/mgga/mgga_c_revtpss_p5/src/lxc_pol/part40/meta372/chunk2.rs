//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1315/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1315(t1032: f64, t4743: f64, t1040: f64, t1647: f64, t3140: f64, t3149: f64, t11921: f64, t247: f64, t4757: f64, t4837: f64, t1659: f64, t3105: f64) -> (f64, f64, f64, f64, f64) {
    let t15816 = t4743 * t1032;
    let t15817 = t15816 * t1040;
    let t15822 = t1647 * t3140;
    let t15823 = t15822 * t3149;
    let t15827 = t247 * t11921 * t4757;
    let t15829 = 0.57165357490759649296e-3_f64 * t4837 * t15827;
    let t15830 = t1659 * t3105;
    (t15817, t15822, t15823, t15829, t15830)
}
