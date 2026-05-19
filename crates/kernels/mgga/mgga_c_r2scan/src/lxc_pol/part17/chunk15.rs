//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 15/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk15<F: Float>(t22: F, t23: F, t6: F, t15: F, t17: F, t19: F, t14: F, rho0: F, rho1: F) -> (F, F, F, F, F, F, F) {
    let t26 = t22 * t6 / t23;
    let t27 = F::new(0.123235e0) * t26;
    let t28 = F::new(0.379785e1) * t15 + t17 + t19 + t27;
    let t31 = F::new(1.0) + F::cast_from(0.16081979498692535067e2_f64) / t28;
    let t32 = F::ln(t31);
    let t34 = F::new(0.621814e-1) * t14 * t32;
    let t35 = rho0 - rho1;
    (t26, t27, t28, t31, t32, t34, t35)
}
