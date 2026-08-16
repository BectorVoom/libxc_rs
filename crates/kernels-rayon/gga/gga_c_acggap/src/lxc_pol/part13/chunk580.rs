//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 580/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk580(t1629: f64, t4210: f64, t1160: f64, t1652: f64, t377: f64, t1170: f64, t151: f64, t1530: f64, t3057: f64, t3059: f64, t3063: f64, t3067: f64, t3071: f64, t4182: f64, t4185: f64, t4188: f64, t4191: f64, t4192: f64, t4194: f64, t4198: f64, t4200: f64, t4203: f64, t4206: f64) -> f64 {
    let t4211 = t1629 * t4210;
    let t4213 = 0.13170898365871023197e1_f64 * t1160 * t4211;
    let t4215 = 0.13170898365871023197e1_f64 * t377 * t1652;
    let t4219 = t4182 - 0.13170898365871023197e1_f64 * t4185 - t4188 + t4191 + t3057 + 0.13170898365871023197e1_f64 * t4192 - 0.65854491829355115987e0_f64 * t1170 * t4194 - 0.39512695097613069591e1_f64 * t4198 * t4200 + 0.39512695097613069591e1_f64 * t1530 * t4203 - 0.65854491829355115987e0_f64 * t151 * t4206 + 0.26341796731742046394e1_f64 * t3059 + t4213 - t4215 + 0.13170898365871023197e1_f64 * t3063 + 0.13170898365871023197e1_f64 * t3067 + 0.65854491829355115987e0_f64 * t3071;
    t4219
}
