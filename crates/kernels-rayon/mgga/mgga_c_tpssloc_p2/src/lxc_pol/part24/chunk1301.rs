//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1301/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1301(t2553: f64, t606: f64, t25: f64, t9516: f64, t2249: f64, t868: f64, t1877: f64, t1915: f64, t22951: f64, t22959: f64, t22961: f64, t22964: f64, t22968: f64, t23286: f64, t25013: f64, t2522: f64, t25372: f64, t4314: f64, t6542: f64, t6666: f64, t6670: f64, t81470: f64, t81476: f64, t81483: f64, t81486: f64, t81489: f64, t81492: f64, t81501: f64) -> f64 {
    let t81505 = t606 * t2553;
    let t81509 = t25 * t9516;
    let t81513 = t2249 * t868;
    let t81520 = 9.0_f64 * t25013 * t81470 + 9.0_f64 / 2.0_f64 * t2522 * t23286 * t6542 + 9.0_f64 * t22959 * t81476 + 3.0_f64 / 2.0_f64 * t1877 * t6666 * t2249 - 9.0_f64 * t81483 * t22961 - 9.0_f64 * t25013 * t81486 - 9.0_f64 / 2.0_f64 * t22959 * t81489 + 3.0_f64 * t25372 * t81492 + 9.0_f64 * t2522 * t6666 * t22964 + 9.0_f64 * t4314 * t6666 * t22951 + 9.0_f64 / 2.0_f64 * t2522 * t1915 * t81501 + 9.0_f64 / 2.0_f64 * t2522 * t1915 * t81505 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t81509 - 3.0_f64 / 2.0_f64 * t1877 * t6670 * t81513 + 9.0_f64 / 2.0_f64 * t2522 * t6666 * t22968;
    t81520
}
