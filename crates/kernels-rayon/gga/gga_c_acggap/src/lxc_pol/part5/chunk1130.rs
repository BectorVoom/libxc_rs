//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1130/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1130(t1035: f64, t6454: f64, t864: f64, t1838: f64, t441: f64, t1170: f64, t1411: f64, t14480: f64, t14485: f64, t14486: f64, t14490: f64, t14491: f64, t14495: f64, t151: f64, t19108: f64, t19112: f64, t19117: f64, t19122: f64, t19129: f64, t407: f64, t4237: f64) -> (f64, f64) {
    let t20220 = t1035 * t6454 * t864;
    let t20228 = t441 * t1838;
    let t20237 = -0.13170898365871023197e1_f64 * t14480 + 0.13170898365871023197e1_f64 * t20220 - 0.52683593463484092788e1_f64 * t19108 + t14485 - 0.13170898365871023197e1_f64 * t14486 + t14490 - 0.26341796731742046394e1_f64 * t19112 - 0.26341796731742046394e1_f64 * t151 * t4237 * t1411 - 0.13170898365871023197e1_f64 * t1170 * t20228 * t407 - 0.39512695097613069591e1_f64 * t19117 + 0.13170898365871023197e1_f64 * t19122 + 0.79025390195226139182e1_f64 * t14491 - 0.13170898365871023197e1_f64 * t19129 + 0.65854491829355115987e0_f64 * t14495;
    (t20228, t20237)
}
