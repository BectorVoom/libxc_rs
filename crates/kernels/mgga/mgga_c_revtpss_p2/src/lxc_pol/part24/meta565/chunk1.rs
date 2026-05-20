//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1715/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1715<F: Float>(t1087: F, t1089: F, t12078: F, t12079: F, t16502: F, t16584: F, t1678: F, t19463: F, t19566: F, t23820: F, t24098: F, t24132: F, t24138: F, t24141: F, t24152: F, t3287: F, t342: F, t380: F, t4857: F, t55988: F, t55991: F, t6368: F, t6379: F, t89245: F, t89355: F, t89503: F) -> F {
    let t89565 = -F::cast_from(0.26341796731742046395e1_f64) * t3287 * t89245 * t1089 - F::cast_from(0.15805078039045227836e2_f64) * t12078 * t89503 * t12079 - F::cast_from(0.15805078039045227836e2_f64) * t55988 * t24138 + F::cast_from(0.79025390195226139183e1_f64) * t55991 * t24141 - F::cast_from(0.79025390195226139183e1_f64) * t16502 * t24132 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t380 * t89355 - F::cast_from(0.79025390195226139183e1_f64) * t19463 * t6368 - F::cast_from(0.79025390195226139183e1_f64) * t16584 * t24152 + F::cast_from(0.79025390195226139183e1_f64) * t19566 * t6379 - F::cast_from(0.79025390195226139183e1_f64) * t4857 * t24098 + F::cast_from(0.26341796731742046395e1_f64) * t1087 * t1678 * t23820 * t1089;
    t89565
}
