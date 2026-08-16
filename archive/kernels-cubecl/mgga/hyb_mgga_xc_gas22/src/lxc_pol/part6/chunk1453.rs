//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1453/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1453<F: Float>(t10851: F, t10852: F, t11187: F, t11188: F, t11190: F, t11259: F, t11260: F, t11618: F, t31692: F, t4: F, t8222: F, t8950: F, t9416: F, t9417: F, t9814: F) -> F {
    let t31707 = t4 * t31692 + F::cast_from(2.0_f64) * t10851 + F::cast_from(2.0_f64) * t10852 + F::cast_from(2.0_f64) * t11187 + F::cast_from(4.0_f64) * t11188 + F::cast_from(2.0_f64) * t11190 + F::cast_from(2.0_f64) * t11259 + F::cast_from(2.0_f64) * t11260 + F::cast_from(2.0_f64) * t11618 + F::cast_from(2.0_f64) * t8222 + F::cast_from(2.0_f64) * t8950 + F::cast_from(2.0_f64) * t9416 + F::cast_from(4.0_f64) * t9417 + F::cast_from(2.0_f64) * t9814;
    t31707
}
