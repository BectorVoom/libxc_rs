//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1064/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1064<F: Float>(t1484: F, t1530: F, t16596: F, t1877: F, t193: F, t202: F, t2057: F, t24339: F, t24344: F, t2522: F, t25365: F, t25374: F, t26739: F, t26744: F, t4119: F, t4255: F, t4303: F, t4314: F, t7110: F, t7114: F, t776: F, t7845: F, t868: F, t870: F) -> F {
    let t26806 = t193 * t202 * t26739 * t870 + F::cast_from(3.0_f64) * t1484 * t2522 * t7110 - t1530 * t1877 * t24339 - F::cast_from(3.0_f64) * t16596 * t2522 * t7114 + F::cast_from(2.0_f64) * t1877 * t24344 * t25374 - t1877 * t26744 * t868 - t1877 * t4303 * t7114 + F::cast_from(3.0_f64) * t2057 * t2522 * t4119 + F::cast_from(6.0_f64) * t2057 * t4255 * t4314 - F::cast_from(3.0_f64) * t2522 * t25365 * t7114 + F::cast_from(3.0_f64) * t2522 * t776 * t7845;
    t26806
}
