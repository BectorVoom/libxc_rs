//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1191/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1191(t35258: f64, t35271: f64, t31029: f64, t31033: f64, t31039: f64, t31041: f64, t31045: f64, t31049: f64, t31060: f64, t31074: f64, t31081: f64, t31083: f64, t31095: f64, t31100: f64, t32677: f64, t35260: f64, t35264: f64, t35273: f64) -> f64 {
    let t37458 = 0.32012600194825403606e-1_f64 * t35258;
    let t37464 = 0.21437009059034868486e-3_f64 * t35271;
    let t37471 = 0.4584375e-1_f64 * t31029 + 0.916875e-1_f64 * t31033 + t32677 + 0.16006300097412701803e-1_f64 * t31039 - 0.85748036236139473944e-3_f64 * t31041 + t37458 - 0.75475421495049964966e-2_f64 * t35260 + 0.62896184579208304138e-3_f64 * t35264 - 0.64311027177104605458e-2_f64 * t31045 + 0.28582678745379824648e-3_f64 * t31049 + 0.18868855373762491241e-2_f64 * t31060 - t37464 + 0.51448821741683684367e-2_f64 * t35273 + 0.34299214494455789578e-2_f64 * t31074 - 0.84046875e-1_f64 * t31081 - 0.5603125e-1_f64 * t31083 - 0.34299214494455789578e-2_f64 * t31095 - 0.85748036236139473944e-2_f64 * t31100;
    t37471
}
