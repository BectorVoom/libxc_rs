//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 906/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk906<F: Float>(t10199: F, t10202: F, t10208: F, t1036: F, t13691: F, t13696: F, t13699: F, t13744: F, t13747: F, t13750: F, t13783: F, t1670: F, t245: F, t2944: F, t2952: F, t3078: F, t3081: F, t4625: F, t4647: F, t4654: F, t934: F) -> F {
    let t13786 = F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t10199 * t13691 - t10202 * t4647 / F::cast_from(4.0_f64) - t3078 * t13696 / F::cast_from(4.0_f64) - t3078 * t13699 / F::cast_from(8.0_f64) + t10208 * t1670 / F::cast_from(4.0_f64) + t3081 * t4625 / F::cast_from(2.0_f64) + t1036 * t13744 / F::cast_from(4.0_f64) - t13747 * t2944 / F::cast_from(8.0_f64) + t13750 * t934 / F::cast_from(2.0_f64) + t4654 * t2952 / F::cast_from(4.0_f64) + t245 * t13783 / F::cast_from(2.0_f64);
    t13786
}
