//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1217/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1217(t310: f64, t9779: f64, t32003: f64, t33535: f64, t8406: f64, t32130: f64, t9029: f64, t8397: f64, t9054: f64, t2146: f64, t32135: f64, t32143: f64, t33778: f64, t36432: f64, t36436: f64, t36439: f64, t36447: f64, t36452: f64, t463: f64, t8004: f64, t8411: f64, t8441: f64, t9003: f64, t9789: f64) -> f64 {
    let t40844 = t310 * t9779;
    let t40849 = t32003 * t33535 * t8406;
    let t40852 = t32130 * t33535 * t9029;
    let t40858 = t8397 * t9054;
    let t40860 = -0.52041769129231196772e1_f64 * t2146 * t8004 * t9789 * t463 + 0.65854491829355115987e0_f64 * t40844 - t36432 - t36436 + t36439 - t36447 - 0.17347256376410398924e1_f64 * t33778 * t8441 + 0.34694512752820797848e1_f64 * t40849 - 0.34694512752820797848e1_f64 * t40852 + 0.65854491829355115987e0_f64 * t32135 - t36452 + 0.34694512752820797848e1_f64 * t32143 - 0.52041769129231196772e1_f64 * t9003 * t8411 + 0.34694512752820797848e1_f64 * t40858;
    t40860
}
