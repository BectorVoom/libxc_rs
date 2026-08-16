//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1721/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1721<F: Float>(t6350: F, t1076: F, t1079: F, t11201: F, t16284: F, t16600: F, t16603: F, t16604: F, t1695: F, t20175: F, t20191: F, t20204: F, t23583: F, t23599: F, t23616: F, t24031: F, t24048: F, t24061: F, t24068: F, t24178: F, t3058: F, t42067: F, t4747: F, t4752: F, t53015: F, t53160: F, t6258: F, t6259: F, t6392: F, t6393: F, t88815: F, t89158: F, t89507: F, t89536: F, t89565: F, t89603: F, t89632: F, t89663: F, t89697: F, t89725: F, t995: F, t996: F) -> F {
    let t89736 = t6350 * t6350;
    let t89740 = -F::cast_from(0.65854491829355115987e0_f64) * t995 * t996 * t89158 - F::cast_from(0.26341796731742046395e1_f64) * t4752 * t24178 - F::cast_from(0.79025390195226139183e1_f64) * t20191 * t6259 - F::cast_from(0.39512695097613069592e1_f64) * t20204 * t6259 - F::cast_from(0.26341796731742046395e1_f64) * t4747 * t23599 + F::cast_from(0.15805078039045227836e2_f64) * t16284 * t24061 - F::cast_from(0.15805078039045227836e2_f64) * t53160 * t24068 - F::cast_from(0.15805078039045227836e2_f64) * t4752 * t24048 + F::cast_from(0.15805078039045227836e2_f64) * t11201 * t1079 * t24031 * t1695 - F::cast_from(0.15805078039045227836e2_f64) * t53015 * t24068 - F::cast_from(0.15805078039045227836e2_f64) * t16603 * t16604 * t23616 + F::cast_from(0.52683593463484092788e1_f64) * t3058 * t996 * t88815 - F::cast_from(0.15805078039045227836e2_f64) * t16600 * t23583 - F::cast_from(0.79025390195226139183e1_f64) * t20175 * t6393 - F::cast_from(0.65854491829355115987e0_f64) * t1076 * t1079 * (t89507 + t89536 + t89565 + t89603 + t89632 + t89663 + t89697 + t89725) + F::cast_from(0.39512695097613069592e1_f64) * t995 * t1079 * t6258 * t6392 + F::cast_from(0.15805078039045227836e2_f64) * t1076 * t42067 * t89736;
    t89740
}
