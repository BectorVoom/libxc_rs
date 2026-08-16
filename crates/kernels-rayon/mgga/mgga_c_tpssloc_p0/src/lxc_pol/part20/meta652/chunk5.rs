//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2405/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2405(t10704: f64, t2836: f64, t49240: f64, t912: f64, t10655: f64, t14422: f64, t2793: f64, t2842: f64, t4396: f64, t10662: f64, t10702: f64, t4399: f64) -> (f64, f64, f64, f64) {
    let t49244 = 0.1551780387578202009e4_f64 * t49240 * t10704 * t2836 * t912;
    let t49256 = 18.0_f64 * t10655 * t14422;
    let t49259 = 18.0_f64 * t2842 * t4396 * t2793;
    let t49262 = 0.57895126195293126241e3_f64 * t10702 * t4399 * t10662;
    (t49244, t49256, t49259, t49262)
}
