//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2013/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2013<F: Float>(t12021: F, t1375: F, t1843: F, t20060: F, t24082: F, t29311: F, t29372: F, t3882: F, t6439: F, t6440: F, t7199: F, t7213: F, t81264: F, t90642: F, t93338: F, t93439: F, t97513: F, t97516: F) -> F {
    let t102523 = -F::cast_from(2.0_f64) * t93338 * t1843 - F::cast_from(0.16449340668482264365e-1_f64) * t97513 + F::cast_from(0.6579736267392905746e-1_f64) * t97516 + F::cast_from(0.3289868133696452873e-1_f64) * t90642 + F::cast_from(2.0_f64) * t24082 * t6440 + t93439 - F::cast_from(6.0_f64) * t1375 * t12021 * t7213 * t6439 + F::cast_from(4.0_f64) * t3882 * t29311 + F::cast_from(2.0_f64) * t20060 * t7199 + F::cast_from(0.52089578783527170489e-1_f64) * t81264 + F::cast_from(2.0_f64) * t3882 * t29372;
    t102523
}
