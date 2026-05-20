//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3442/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3442<F: Float>(t1071: F, t19462: F, t19856: F, t378: F, t1647: F, t4930: F, t3059: F, t6244: F, t1000: F, t1079: F, t1097: F, t11128: F, t11195: F, t16317: F, t16374: F, t1652: F, t16603: F, t19428: F, t20168: F, t20215: F, t3043: F, t3047: F, t42060: F, t4772: F, t4941: F, t5015: F, t53208: F, t6259: F, t6345: F, t6351: F, t989: F, t995: F, t996: F) -> (F, F) {
    let t64629 = t19462 * t1071;
    let t64636 = t19856 * t378;
    let t64639 = t1647 * t4930;
    let t64647 = t6244 * t3059;
    let t64661 = F::cast_from(0.65854491829355115987e0_f64) * t3043 * t6345 - F::cast_from(0.13170898365871023197e1_f64) * t64629 * t1000 + F::cast_from(0.13170898365871023197e1_f64) * t11195 * t6351 + F::cast_from(0.26341796731742046394e1_f64) * t16374 * t4941 - F::cast_from(0.13170898365871023197e1_f64) * t64636 * t1097 - F::cast_from(0.26341796731742046394e1_f64) * t64639 * t1097 + F::cast_from(0.13170898365871023197e1_f64) * t989 * t20168 - F::cast_from(0.26341796731742046394e1_f64) * t16603 * t19428 * t16317 + F::cast_from(0.15805078039045227836e2_f64) * t42060 * t996 * t64647 + F::cast_from(0.26341796731742046394e1_f64) * t3047 * t20215 + F::cast_from(0.26341796731742046394e1_f64) * t995 * t1079 * t4772 * t5015 - F::cast_from(0.13170898365871023197e1_f64) * t53208 * t1652 - F::cast_from(0.13170898365871023197e1_f64) * t11128 * t6259;
    (t64647, t64661)
}
