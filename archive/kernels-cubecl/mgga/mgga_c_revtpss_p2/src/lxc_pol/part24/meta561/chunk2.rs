//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1687/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1687<F: Float>(t341: F, t88660: F, t88673: F, t1076: F, t1079: F, t11121: F, t11201: F, t16284: F, t1651: F, t1695: F, t1696: F, t20211: F, t23583: F, t23598: F, t23603: F, t23607: F, t23617: F, t24047: F, t24177: F, t24178: F, t3058: F, t3269: F, t386: F, t4747: F, t4778: F, t4935: F, t6244: F, t6251: F, t6258: F, t6350: F, t80833: F, t80992: F, t88628: F, t88646: F, t995: F, t996: F) -> (F, F) {
    let t88675 = (t88660 + t88673) * t341;
    let t88682 = F::cast_from(0.26341796731742046395e1_f64) * t995 * t1079 * t24177 * t1651 + F::cast_from(0.15805078039045227836e2_f64) * t3058 * t3269 * t6244 * t6350 - F::cast_from(0.15805078039045227836e2_f64) * t4778 * t23607 + F::cast_from(0.79025390195226139183e1_f64) * t20211 * t6251 + F::cast_from(0.15805078039045227836e2_f64) * t4935 * t23603 + F::cast_from(0.15805078039045227836e2_f64) * t995 * t11121 * t24047 * t1651 - F::cast_from(0.79025390195226139183e1_f64) * t80992 * t1696 + F::cast_from(0.39512695097613069591e1_f64) * t1076 * t3269 * t88628 - F::cast_from(0.26341796731742046395e1_f64) * t80833 * t1696 - F::cast_from(0.26341796731742046395e1_f64) * t4935 * t24178 - F::cast_from(0.79025390195226139183e1_f64) * t995 * t3269 * t6258 * t6350 + F::cast_from(0.26341796731742046395e1_f64) * t995 * t1079 * t23598 * t1695 - F::cast_from(0.15805078039045227836e2_f64) * t4747 * t23607 - F::cast_from(0.23707617058567841754e2_f64) * t11201 * t996 * t88646 + F::cast_from(0.65854491829355115987e0_f64) * t88675 * t386 + F::cast_from(0.79025390195226139183e1_f64) * t4778 * t23617 - F::cast_from(0.15805078039045227836e2_f64) * t16284 * t23583;
    (t88675, t88682)
}
