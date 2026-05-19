//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 630/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk630<F: Float>(t7899: F, t8690: F, t2007: F, t383: F, t528: F, t7977: F, t7944: F, t1655: F, t7946: F, t7948: F, t7950: F, t7952: F, t7957: F, t7961: F, t7964: F, t7968: F, t7971: F, t7975: F) -> (F, F, F, F) {
    let t8691 = t8690 * t7899;
    let t8693 = t2007 * t383;
    let t8696 = t528 * t7977;
    let t8698 = F::cast_from(0.44934037037037037036e0_f64) * t7944;
    let t8709 = F::new(0.1760655e0) * t8691 - F::new(0.352131e0) * t8693 * t1655 + F::new(0.234754e0) * t8696 - t8698 - F::cast_from(0.19257444444444444444e0_f64) * t7946 + F::cast_from(0.9628722222222222222e-1_f64) * t7948 - F::cast_from(0.28886166666666666666e0_f64) * t7950 + F::cast_from(0.14443083333333333333e0_f64) * t7952 - F::cast_from(0.1604787037037037037e0_f64) * t7957 + F::cast_from(0.57772333333333333332e0_f64) * t7961 - F::cast_from(0.28886166666666666666e0_f64) * t7964 - F::cast_from(0.86658499999999999998e0_f64) * t7968 + F::cast_from(0.86658499999999999998e0_f64) * t7971 - F::cast_from(0.14443083333333333333e0_f64) * t7975;
    (t8691, t8693, t8696, t8709)
}
