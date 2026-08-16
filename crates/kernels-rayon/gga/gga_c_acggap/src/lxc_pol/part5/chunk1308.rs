//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1308/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1308(t14230: f64, t3073: f64, t6465: f64, t4180: f64, t6469: f64, t377: f64, t6507: f64, t119: f64, t1247: f64, t14501: f64, t14503: f64, t151: f64, t1530: f64, t1629: f64, t182: f64, t1839: f64, t19133: f64, t19144: f64, t19149: f64, t19152: f64, t22127: f64, t23821: f64, t24340: f64, t2925: f64, t6551: f64, t930: f64) -> f64 {
    let t24359 = t3073 * t6465 * t14230;
    let t24361 = t4180 * t6469;
    let t24363 = t377 * t6507;
    let t24368 = 0.65854491829355115987e0_f64 * t119 * t182 * t24340 - 0.26341796731742046394e1_f64 * t19133 - 0.65854491829355115987e0_f64 * t151 * t2925 * t1839 + t14501 - 0.65854491829355115987e0_f64 * t151 * t6551 * t930 + 0.26341796731742046394e1_f64 * t151 * t1247 * t22127 + 0.13170898365871023197e1_f64 * t14503 + 0.26341796731742046394e1_f64 * t1530 * t1629 * t23821 - 0.13170898365871023197e1_f64 * t24359 + 0.52683593463484092788e1_f64 * t24361 - 0.26341796731742046394e1_f64 * t24363 - 0.26341796731742046394e1_f64 * t19144 + 0.52683593463484092788e1_f64 * t19149 - 0.79025390195226139182e1_f64 * t19152;
    t24368
}
