//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 673/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk673(t1424: f64, t1445: f64, t213: f64, t3894: f64, t3898: f64, t3901: f64, t3904: f64, t3910: f64, t3912: f64, t3918: f64, t3922: f64, t4067: f64, t4071: f64, t4078: f64, t4132: f64, t561: f64) -> f64 {
    let t4135 = t3894 - t3898 - 0.10975748638225852664e-1_f64 * t3901 + 0.10975748638225852664e-1_f64 * t3904 + t3910 + 0.19514881078765566038e-1_f64 * t3912 - 0.19514881078765566038e-1_f64 * t3918 - t3922 + 0.65854491829355115987e0_f64 * t213 * t4067 * t561 - 0.13170898365871023197e1_f64 * t4071 * t1445 + 0.13170898365871023197e1_f64 * t1424 * t4078 - 0.65854491829355115987e0_f64 * t1424 * t4132;
    t4135
}
