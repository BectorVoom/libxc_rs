//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3563/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3563(t1076: f64, t16239: f64, t16287: f64, t16362: f64, t1647: f64, t1652: f64, t19342: f64, t19351: f64, t20152: f64, t20188: f64, t3052: f64, t3063: f64, t3270: f64, t3326: f64, t42067: f64, t42107: f64, t43637: f64, t43642: f64, t4747: f64, t4947: f64, t53058: f64, t53157: f64, t55421: f64, t6245: f64, t6350: f64) -> f64 {
    let t68067 = -0.13170898365871023197e1_f64 * t3052 * t20152 + 0.13170898365871023197e1_f64 * t1647 * t16239 - 0.13170898365871023197e1_f64 * t53058 * t1652 + 0.13170898365871023197e1_f64 * t42107 * t6245 - 0.79025390195226139182e1_f64 * t43637 * t20188 - 0.26341796731742046394e1_f64 * t55421 * t1652 + 0.52683593463484092788e1_f64 * t16362 * t4947 - 0.65854491829355115987e0_f64 * t19351 * t3326 + 0.15805078039045227836e2_f64 * t1076 * t42067 * t6350 * t3270 - 0.13170898365871023197e1_f64 * t4747 * t16287 - 0.13170898365871023197e1_f64 * t53157 * t1652 - 0.26341796731742046394e1_f64 * t3063 * t19342 + 0.13170898365871023197e1_f64 * t43642 * t6245;
    t68067
}
