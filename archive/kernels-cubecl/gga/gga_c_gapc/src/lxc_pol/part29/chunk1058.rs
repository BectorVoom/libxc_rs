//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1058/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1058<F: Float>(t11299: F, t11292: F, t11287: F, t11280: F, t8601: F, t8613: F, t11609: F, t1611: F, t12006: F, t699: F, t1617: F, t3721: F, t4915: F) -> (F, F, F, F, F, F, F, F) {
    let t33098 = F::cast_from(4.0_f64) * t11299;
    let t33099 = F::cast_from(12.0_f64) * t11292;
    let t33100 = F::cast_from(4.0_f64) * t11287;
    let t33101 = F::cast_from(2.0_f64) * t11280;
    let t33103 = F::cast_from(8.0_f64) * t8601 * t8613;
    let t33105 = F::cast_from(2.0_f64) * t1611 * t11609;
    let t33106 = t699 * t12006;
    let t33110 = F::cast_from(6.0_f64) * t4915 * t3721 * t1617;
    (t33098, t33099, t33100, t33101, t33103, t33105, t33106, t33110)
}
