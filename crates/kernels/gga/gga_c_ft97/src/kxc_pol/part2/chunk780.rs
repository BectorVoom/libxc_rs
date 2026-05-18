//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 780/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk780<F: Float>(t11167: F, t11177: F, t11183: F, t11192: F, t11202: F, t12233: F, t7946: F, t7948: F, t7950: F, t7952: F, t8698: F, t637: F, t639: F) -> F {
    let t12234 = -F::new(0.9628722222222222222e-1) * t7950 + F::new(0.10591594444444444444e1) * t11177 - F::new(0.28886166666666666666e0) * t11202 - t8698 + F::new(0.3209574074074074074e-1) * t7948 - F::new(0.12838296296296296296e0) * t7946 + F::new(0.4814361111111111111e-1) * t7952 + F::new(0.57772333333333333332e0) * t11183 - F::new(0.86658499999999999998e0) * t11192 - F::new(0.6419148148148148148e-1) * t11167 + t12233;
    let t12236 = t637 * t639 * t12234;
    t12236
}
