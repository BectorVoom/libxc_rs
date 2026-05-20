//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3563/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3563<F: Float>(t1076: F, t16239: F, t16287: F, t16362: F, t1647: F, t1652: F, t19342: F, t19351: F, t20152: F, t20188: F, t3052: F, t3063: F, t3270: F, t3326: F, t42067: F, t42107: F, t43637: F, t43642: F, t4747: F, t4947: F, t53058: F, t53157: F, t55421: F, t6245: F, t6350: F) -> F {
    let t68067 = -F::cast_from(0.13170898365871023197e1_f64) * t3052 * t20152 + F::cast_from(0.13170898365871023197e1_f64) * t1647 * t16239 - F::cast_from(0.13170898365871023197e1_f64) * t53058 * t1652 + F::cast_from(0.13170898365871023197e1_f64) * t42107 * t6245 - F::cast_from(0.79025390195226139182e1_f64) * t43637 * t20188 - F::cast_from(0.26341796731742046394e1_f64) * t55421 * t1652 + F::cast_from(0.52683593463484092788e1_f64) * t16362 * t4947 - F::cast_from(0.65854491829355115987e0_f64) * t19351 * t3326 + F::cast_from(0.15805078039045227836e2_f64) * t1076 * t42067 * t6350 * t3270 - F::cast_from(0.13170898365871023197e1_f64) * t4747 * t16287 - F::cast_from(0.13170898365871023197e1_f64) * t53157 * t1652 - F::cast_from(0.26341796731742046394e1_f64) * t3063 * t19342 + F::cast_from(0.13170898365871023197e1_f64) * t43642 * t6245;
    t68067
}
