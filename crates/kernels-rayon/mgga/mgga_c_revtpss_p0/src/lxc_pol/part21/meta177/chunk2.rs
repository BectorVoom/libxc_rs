//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1103/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1103(t1399: f64, t1437: f64, t213: f64, t3924: f64, t4004: f64, t4057: f64, t4066: f64, t4082: f64, t4085: f64, t4090: f64, t4094: f64, t4099: f64, t4105: f64, t4109: f64, t4113: f64, t4114: f64, t4118: f64, t546: f64, t820: f64) -> f64 {
    let t4131 = t4082 - t4085 + 0.10975748638225852664e-1_f64 * t4090 - 0.10975748638225852664e-1_f64 * t4094 + t4099 - 0.19514881078765566038e-1_f64 * t4105 + 0.19514881078765566038e-1_f64 * t4109 - t4113 + 0.13170898365871023197e1_f64 * t820 * t4114 * t4004 - 0.13170898365871023197e1_f64 * t820 * t4118 * t1399 - 0.65854491829355115987e0_f64 * t820 * t1437 * t4057 - 0.65854491829355115987e0_f64 * t820 * t1437 * t3924 + 0.65854491829355115987e0_f64 * t213 * t546 * t4066;
    t4131
}
