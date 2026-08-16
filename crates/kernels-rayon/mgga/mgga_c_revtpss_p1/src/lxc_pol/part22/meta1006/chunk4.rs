//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3442/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3442(t1071: f64, t19462: f64, t19856: f64, t378: f64, t1647: f64, t4930: f64, t3059: f64, t6244: f64, t1000: f64, t1079: f64, t1097: f64, t11128: f64, t11195: f64, t16317: f64, t16374: f64, t1652: f64, t16603: f64, t19428: f64, t20168: f64, t20215: f64, t3043: f64, t3047: f64, t42060: f64, t4772: f64, t4941: f64, t5015: f64, t53208: f64, t6259: f64, t6345: f64, t6351: f64, t989: f64, t995: f64, t996: f64) -> (f64, f64) {
    let t64629 = t19462 * t1071;
    let t64636 = t19856 * t378;
    let t64639 = t1647 * t4930;
    let t64647 = t6244 * t3059;
    let t64661 = 0.65854491829355115987e0_f64 * t3043 * t6345 - 0.13170898365871023197e1_f64 * t64629 * t1000 + 0.13170898365871023197e1_f64 * t11195 * t6351 + 0.26341796731742046394e1_f64 * t16374 * t4941 - 0.13170898365871023197e1_f64 * t64636 * t1097 - 0.26341796731742046394e1_f64 * t64639 * t1097 + 0.13170898365871023197e1_f64 * t989 * t20168 - 0.26341796731742046394e1_f64 * t16603 * t19428 * t16317 + 0.15805078039045227836e2_f64 * t42060 * t996 * t64647 + 0.26341796731742046394e1_f64 * t3047 * t20215 + 0.26341796731742046394e1_f64 * t995 * t1079 * t4772 * t5015 - 0.13170898365871023197e1_f64 * t53208 * t1652 - 0.13170898365871023197e1_f64 * t11128 * t6259;
    (t64647, t64661)
}
