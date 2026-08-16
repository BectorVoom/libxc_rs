//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 884/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk884(t1393: f64, t2036: f64, t2040: f64, t2096: f64, t2165: f64, t27888: f64, t31753: f64, t31761: f64, t31769: f64, t31771: f64, t31774: f64, t31778: f64, t31832: f64, t32350: f64, t672: f64, t7040: f64, t7050: f64, t7218: f64, t7266: f64, t7408: f64, t8690: f64, t8840: f64) -> f64 {
    let t32390 = t1393 * t8840 - t2036 * t7408 - 2.0_f64 * t2040 * t27888 + t2096 * t31832 - t2165 * t7040 - 2.0_f64 * t32350 * t672 - 2.0_f64 * t7050 * t7266 + t7218 * t8690 - t31753 + t31761 - t31769 - t31771 - t31774 + t31778;
    t32390
}
