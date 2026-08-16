//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 965/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk965<F: Float>(t20173: F, t31814: F, t31817: F, t1874: F, t91854: F, t23938: F, t6525: F, t1873: F, t2311: F, t2040: F, t2314: F, t31744: F) -> (F, F, F, F, F, F, F) {
    let t114529 = F::cast_from(54.0_f64) * t20173 * t31814;
    let t114531 = F::cast_from(54.0_f64) * t20173 * t31817;
    let t114541 = F::cast_from(4.0_f64) * t91854 * t1874;
    let t114543 = F::cast_from(4.0_f64) * t23938 * t6525;
    let t114552 = t2311 * t1873;
    let t114554 = F::cast_from(2.0_f64) * t114552 * t2040;
    let t114559 = F::cast_from(4.0_f64) * t2314 * t31744;
    (t114529, t114531, t114541, t114543, t114552, t114554, t114559)
}
