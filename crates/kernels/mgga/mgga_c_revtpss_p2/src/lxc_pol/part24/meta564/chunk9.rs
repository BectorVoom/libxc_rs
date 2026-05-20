//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1711/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1711<F: Float>(t1076: F, t1079: F, t16312: F, t16313: F, t1647: F, t16600: F, t1695: F, t1696: F, t19351: F, t20178: F, t225: F, t23599: F, t23617: F, t23620: F, t23621: F, t24044: F, t24048: F, t24061: F, t24177: F, t3058: F, t3269: F, t342: F, t385: F, t4747: F, t4778: F, t4935: F, t6235: F, t6244: F, t6345: F, t6351: F, t6392: F, t6393: F, t80983: F, t81052: F, t89355: F) -> F {
    let t89397 = F::cast_from(0.65854491829355115987e0_f64) * t342 * t89355 * t225 * t385 - F::cast_from(0.26341796731742046395e1_f64) * t81052 * t1696 - F::cast_from(0.39512695097613069592e1_f64) * t20178 * t6393 - F::cast_from(0.79025390195226139183e1_f64) * t3058 * t1079 * t6244 * t6392 + F::cast_from(0.79025390195226139183e1_f64) * t4778 * t23621 + F::cast_from(0.79025390195226139183e1_f64) * t4747 * t23617 - F::cast_from(0.79025390195226139183e1_f64) * t80983 * t1696 + F::cast_from(0.15805078039045227836e2_f64) * t16600 * t24061 + F::cast_from(0.39512695097613069592e1_f64) * t6235 * t6345 - F::cast_from(0.39512695097613069592e1_f64) * t19351 * t6393 + F::cast_from(0.79025390195226139183e1_f64) * t4747 * t23621 + F::cast_from(0.26341796731742046395e1_f64) * t1647 * t24044 - F::cast_from(0.15805078039045227836e2_f64) * t4935 * t24048 + F::cast_from(0.52683593463484092788e1_f64) * t1076 * t3269 * t1695 * t24177 + F::cast_from(0.79025390195226139183e1_f64) * t19351 * t6351 - F::cast_from(0.26341796731742046395e1_f64) * t4778 * t23599 - F::cast_from(0.15805078039045227836e2_f64) * t16312 * t16313 * t23620;
    t89397
}
