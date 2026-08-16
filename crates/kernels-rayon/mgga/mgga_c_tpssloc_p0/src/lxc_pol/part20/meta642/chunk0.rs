//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2350/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2350(t344: f64, t42308: f64, t60: f64, t1597: f64, t341: f64, t10245: f64, t13847: f64, t2986: f64, t13931: f64, t2987: f64, t135: f64, t13933: f64, t973: f64) -> (f64, f64, f64, f64, f64) {
    let t48180 = t60 * t42308 * t344;
    let t48184 = t341 * t1597;
    let t48189 = t2986 * t13847 * t10245;
    let t48191 = t2987 * t13931;
    let t48207 = t973 * t135 * t13933;
    (t48180, t48184, t48189, t48191, t48207)
}
