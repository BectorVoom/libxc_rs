//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1158/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1158<F: Float>(t10021: F, t1336: F, t1361: F, t1369: F, t119: F, t12286: F, t12293: F, t12297: F, t12361: F, t1315: F, t1343: F, t210: F, t3733: F, t3783: F, t39622: F, t39892: F, t40012: F, t40019: F, t40022: F, t40025: F, t40026: F, t40035: F, t40044: F, t40047: F, t40052: F, t40054: F, t820: F) -> F {
    let t40059 = t1336 * t1361 * t10021;
    let t40060 = t40059 * t1369;
    let t40062 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t40012 - t1315 * t210 * t119 * t39892 / F::cast_from(48.0_f64) + F::cast_from(35.0_f64) / F::cast_from(12.0_f64) * t40019 + F::cast_from(7.0_f64) / F::cast_from(3.0_f64) * t40022 + F::cast_from(5.0_f64) / F::cast_from(4.0_f64) * t40025 * t210 * t119 * t40026 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t3733 * t210 * t119 * t39622 - t40035 * t12293 / F::cast_from(128.0_f64) + t12286 * t12297 / F::cast_from(128.0_f64) + t40044 * t1343 * t820 * t40047 / F::cast_from(128.0_f64) + F::cast_from(35.0_f64) / F::cast_from(96.0_f64) * t40052 + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t40054 - t3783 * t12361 / F::cast_from(192.0_f64) + F::cast_from(595.0_f64) / F::cast_from(648.0_f64) * t40060;
    t40062
}
