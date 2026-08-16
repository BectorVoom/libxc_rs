//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3461/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3461<F: Float>(t4772: F, t11249: F, t6299: F, t1024: F, t1082: F, t11788: F, t11940: F, t12073: F, t15670: F, t16152: F, t16183: F, t16399: F, t16433: F, t16449: F, t16485: F, t16496: F, t1651: F, t16566: F, t16568: F, t19556: F, t19580: F, t19617: F, t3059: F, t3143: F, t3204: F, t378: F, t43154: F, t43453: F, t4893: F, t4954: F, t4977: F, t4981: F, t4982: F, t55330: F, t55701: F, t55764: F, t6244: F, t64647: F, t64772: F, t65096: F) -> (F, F, F) {
    let t65122 = t4772 * t4772;
    let t65144 = t6299 * t11249;
    let t65150 = -F::cast_from(0.15805078039045227836e2_f64) * t55330 * t55764 * t16152 + F::cast_from(0.52683593463484092788e1_f64) * t15670 * t16399 + F::cast_from(0.13170898365871023197e1_f64) * t3204 * t12073 * t6244 - F::cast_from(0.39512695097613069591e1_f64) * t11940 * t1082 * t64772 - F::cast_from(0.26341796731742046394e1_f64) * t1024 * t16449 * t4772 - F::cast_from(0.26341796731742046394e1_f64) * t55701 * t4977 + F::cast_from(0.13170898365871023197e1_f64) * t3204 * t19556 * t3059 + F::cast_from(0.26341796731742046394e1_f64) * t3204 * t1082 * t65122 + F::cast_from(0.10536718692696818558e2_f64) * t65096 * t3143 * t378 * t1651 * t16433 + F::cast_from(0.26341796731742046394e1_f64) * t15670 * t16485 + F::cast_from(0.15805078039045227836e2_f64) * t43154 * t1082 * t64647 + F::cast_from(0.26341796731742046394e1_f64) * t4954 * t16496 + F::cast_from(0.26341796731742046394e1_f64) * t11788 * t19617 + F::cast_from(0.26341796731742046394e1_f64) * t4981 * t4893 * t4982 * t16183 + F::cast_from(0.65854491829355115987e0_f64) * t16566 * t65144 * t16568 + F::cast_from(0.13170898365871023197e1_f64) * t43453 * t19580;
    (t65122, t65144, t65150)
}
