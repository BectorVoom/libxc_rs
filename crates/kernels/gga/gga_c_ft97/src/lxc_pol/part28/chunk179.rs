//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 179/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk179<F: Float>(t632: F, t72: F, t920: F, t1002: F, t641: F, t927: F, t637: F, t639: F, t629: F, t631: F, t184: F, t21: F) -> (F, F, F, F, F, F) {
    let t1068 = t72 * t632 * t920;
    let t1073 = F::new(0.234754e0) * t1002 - t641 - F::cast_from(0.14443083333333333333e0_f64) * t927;
    let t1075 = t637 * t639 * t1073;
    let t1078 = t629 + t631 * t1068 / F::new(6.0) + t631 * t1075 / F::new(2.0);
    let t1079 = t1078 * t184;
    let t1080 = t1079 * t21;
    (t1068, t1073, t1075, t1078, t1079, t1080)
}
