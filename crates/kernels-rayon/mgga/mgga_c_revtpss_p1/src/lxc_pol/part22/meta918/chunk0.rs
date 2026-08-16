//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3127/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3127(t12046: f64, t1647: f64, t16551: f64, t989: f64, t12153: f64, t4746: f64, t16237: f64, t359: f64, t15654: f64, t3286: f64, t16543: f64, t3046: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55599 = t1647 * t12046;
    let t55632 = t989 * t16551;
    let t55646 = t4746 * t12153;
    let t55649 = t359 * t16237;
    let t55685 = t15654 * t3286;
    let t55701 = t3046 * t16543;
    (t55599, t55632, t55646, t55649, t55685, t55701)
}
