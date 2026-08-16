//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1130/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1130<F: Float>(t1035: F, t6454: F, t864: F, t1838: F, t441: F, t1170: F, t1411: F, t14480: F, t14485: F, t14486: F, t14490: F, t14491: F, t14495: F, t151: F, t19108: F, t19112: F, t19117: F, t19122: F, t19129: F, t407: F, t4237: F) -> (F, F) {
    let t20220 = t1035 * t6454 * t864;
    let t20228 = t441 * t1838;
    let t20237 = -F::cast_from(0.13170898365871023197e1_f64) * t14480 + F::cast_from(0.13170898365871023197e1_f64) * t20220 - F::cast_from(0.52683593463484092788e1_f64) * t19108 + t14485 - F::cast_from(0.13170898365871023197e1_f64) * t14486 + t14490 - F::cast_from(0.26341796731742046394e1_f64) * t19112 - F::cast_from(0.26341796731742046394e1_f64) * t151 * t4237 * t1411 - F::cast_from(0.13170898365871023197e1_f64) * t1170 * t20228 * t407 - F::cast_from(0.39512695097613069591e1_f64) * t19117 + F::cast_from(0.13170898365871023197e1_f64) * t19122 + F::cast_from(0.79025390195226139182e1_f64) * t14491 - F::cast_from(0.13170898365871023197e1_f64) * t19129 + F::cast_from(0.65854491829355115987e0_f64) * t14495;
    (t20228, t20237)
}
