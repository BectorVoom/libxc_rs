//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1041/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1041(t1259: f64, t3936: f64, t11240: f64, t11242: f64, t11245: f64, t11263: f64, t11266: f64, t11292: f64, t11295: f64, t11357: f64, t11359: f64, t11361: f64, t11367: f64, t1306: f64, t135: f64, t273: f64, t6362: f64, t9759: f64) -> (f64, f64) {
    let t11541 = t3936 * t1259;
    let t11549 = 2.0_f64 * t11541 * t135 * t273 * t6362 - 3.0_f64 * t1259 * t1306 * t9759 + t11240 + t11242 - t11245 + t11263 + t11266 - t11292 + t11295 - t11357 - t11359 - t11361 - t11367;
    (t11541, t11549)
}
