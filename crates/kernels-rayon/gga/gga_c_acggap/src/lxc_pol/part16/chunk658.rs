//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 658/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk658(t180: f64, t1838: f64, t1533: f64, t1539: f64, t1160: f64, t1530: f64, t3057: f64, t3059: f64, t3067: f64, t3078: f64, t3091: f64, t3104: f64, t4191: f64, t4192: f64, t4213: f64, t4215: f64, t4228: f64, t4230: f64, t4231: f64, t4234: f64) -> (f64, f64) {
    let t6482 = t180 * t1838;
    let t6483 = t6482 * t1533;
    let t6489 = t6482 * t1539;
    let t6490 = t1160 * t6489;
    let t6493 = t4191 + t3057 + 0.26341796731742046394e1_f64 * t4192 + 0.13170898365871023197e1_f64 * t3059 + t4213 - t4215 + 0.13170898365871023197e1_f64 * t1530 * t6483 + 0.65854491829355115987e0_f64 * t3067 + 0.13170898365871023197e1_f64 * t3078 - 0.13170898365871023197e1_f64 * t3091 - t4228 + 0.65854491829355115987e0_f64 * t6490 - t4230 - 0.13170898365871023197e1_f64 * t4231 - t4234 - t3104;
    (t6482, t6493)
}
