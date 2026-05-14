//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 595/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk595<F: Float>(t528: F, t7977: F, t7944: F, t1655: F, t7946: F, t7948: F, t7950: F, t7952: F, t7957: F, t7961: F, t7964: F, t7968: F, t7971: F, t7975: F, t8691: F, t8693: F) -> (F, F) {
    let t8696 = t528 * t7977;
    let t8698 = 0.44934037037037037036e0 * t7944;
    let t8709 = 0.1760655e0 * t8691 - 0.352131e0 * t8693 * t1655 + 0.234754e0 * t8696 - t8698 - 0.19257444444444444444e0 * t7946 + 0.9628722222222222222e-1 * t7948 - 0.28886166666666666666e0 * t7950 + 0.14443083333333333333e0 * t7952 - 0.1604787037037037037e0 * t7957 + 0.57772333333333333332e0 * t7961 - 0.28886166666666666666e0 * t7964 - 0.86658499999999999998e0 * t7968 + 0.86658499999999999998e0 * t7971 - 0.14443083333333333333e0 * t7975;
    (t8696, t8709)
}
