//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1183/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1183<F: Float>(t2268: F, t2304: F, t31936: F, t10242: F, t1595: F, t1063: F, t21042: F, t2765: F, t25955: F, t894: F, t20013: F, t2854: F, t6320: F) -> (F, F, F, F, F) {
    let t31939 = F::cast_from(0.39837009289946609438e0_f64) * t2268 * t2304 * t31936;
    let t31942 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t1595 * t10242;
    let t31945 = F::cast_from(0.85365019907028448797e-1_f64) * t1063 * t2765 * t21042;
    let t31948 = F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t894 * t25955;
    let t31952 = F::cast_from(0.17073003981405689759e0_f64) * t2268 * t6320 * t2854 * t20013;
    (t31939, t31942, t31945, t31948, t31952)
}
