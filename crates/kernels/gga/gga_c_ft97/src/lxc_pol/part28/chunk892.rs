//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 892/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk892<F: Float>(t1359: F, t574: F, t6725: F, t1060: F, t2185: F, t7312: F, t1053: F, t7339: F, t605: F, t33096: F, t33101: F, t34811: F, t34815: F, t34820: F, t34825: F, t34829: F, t34833: F, t34837: F, t34841: F, t34846: F) -> (F, F, F, F, F) {
    let t35118 = t574 * t6725 * t1359;
    let t35122 = t2185 * t1060 * t7312;
    let t35125 = t7339 * t1053;
    let t35127 = t574 * t605 * t35125;
    let t35138 = t34811 / F::cast_from(2.0_f64) + t33096 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t34815 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t34820 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t34825 - t34829 / F::cast_from(6.0_f64) - t33101 - t34833 / F::cast_from(9.0_f64) - t34837 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t34841 + t34846 / F::cast_from(12.0_f64);
    (t35118, t35122, t35125, t35127, t35138)
}
