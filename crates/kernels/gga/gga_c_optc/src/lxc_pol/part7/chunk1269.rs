//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1269/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1269<F: Float>(t26150: F, t26152: F, t26156: F, t26160: F, t26163: F, t26168: F, t26170: F, t26173: F, t26175: F, t26179: F, t26181: F, t3075: F, t8743: F) -> (F, F) {
    let t26182 = -t26150 + t26152 - t26156 - t26160 + t26163 + t26168 + t26170 - t26173 - t26175 - t26179 - t26181;
    let t26184 = F::new(0.1038945353962551798e3) * t8743 * t3075;
    (t26182, t26184)
}
