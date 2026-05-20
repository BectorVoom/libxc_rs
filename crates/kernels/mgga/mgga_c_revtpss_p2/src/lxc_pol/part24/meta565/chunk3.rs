//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1717/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1717<F: Float>(t1024: F, t1082: F, t1089: F, t12149: F, t1685: F, t1692: F, t19569: F, t19608: F, t23959: F, t24084: F, t24123: F, t24126: F, t3287: F, t55599: F, t55747: F, t55887: F, t79863: F, t88885: F, t89035: F, t89158: F, t89240: F) -> F {
    let t89632 = F::cast_from(0.26341796731742046395e1_f64) * t23959 * t1692 - F::cast_from(0.26341796731742046395e1_f64) * t79863 * t1685 - F::cast_from(0.79025390195226139184e1_f64) * t19608 * t24084 - F::cast_from(0.26341796731742046395e1_f64) * t3287 * t88885 * t1089 - F::cast_from(0.39512695097613069592e1_f64) * t3287 * t89240 * t1089 - F::cast_from(0.79025390195226139184e1_f64) * t19569 * t24084 + F::cast_from(0.26341796731742046395e1_f64) * t55599 * t24123 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t1082 * t89158 + F::cast_from(0.15805078039045227836e2_f64) * t55747 * t24126 + F::cast_from(0.15805078039045227836e2_f64) * t55887 * t24126 + F::cast_from(0.79025390195226139183e1_f64) * t12149 * t89035 * t1089;
    t89632
}
