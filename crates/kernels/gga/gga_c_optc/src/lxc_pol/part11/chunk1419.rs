//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1419/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1419<F: Float>(t1472: F, t1484: F, t17787: F, t17838: F, t34813: F, t34816: F, t44909: F, t5203: F, t53152: F, t53155: F, t59088: F, t59152: F, t59154: F, t59160: F, t59162: F, t59165: F, t59169: F, t59171: F, t59173: F, t59176: F, t59179: F, t59181: F) -> F {
    let t59379 = -t59088 - t59152 - t59154 - t59160 + F::cast_from(0.23392893589820816284e1_f64) * t53152 * t1484 + F::new(4.0) * t53155 * t1472 - t59162 + t59165 + t59169 - F::cast_from(0.70178680769462448852e1_f64) * t44909 * t5203 - F::cast_from(0.4155781415850207192e3_f64) * t34813 * t17787 + F::cast_from(0.82765347514623860983e4_f64) * t34816 * t17838 + t59171 + t59173 - t59176 - t59179 - t59181;
    t59379
}
