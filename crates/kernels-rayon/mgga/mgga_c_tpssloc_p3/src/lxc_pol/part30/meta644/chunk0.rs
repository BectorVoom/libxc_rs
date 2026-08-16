//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2055/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2055(t25641: f64, t82892: f64, t25638: f64, t6735: f64, t23418: f64, t4669: f64, t13765: f64, t23419: f64, t10469: f64, t23470: f64, t3: f64, t82986: f64) -> (f64, f64, f64, f64, f64) {
    let t88488 = 0.20186378047070195428e-3_f64 * t82892 * t25641;
    let t88503 = 0.20186378047070195428e-3_f64 * t25638 * t6735;
    let t88513 = t4669 * t23418;
    let t88517 = t23419 * t13765 / 1728.0_f64;
    let t88537 = t82986 * t3 * t23470 * t10469;
    (t88488, t88503, t88513, t88517, t88537)
}
