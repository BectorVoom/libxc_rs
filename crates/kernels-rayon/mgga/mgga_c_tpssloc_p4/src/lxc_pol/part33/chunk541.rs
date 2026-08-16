//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 541/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk541(t1409: f64, t2989: f64, t2987: f64, t344: f64, t135: f64, t1599: f64, t973: f64, t1597: f64, t340: f64, t974: f64, t1604: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4514 = t2989 * t1409;
    let t4518 = t2987 * t344;
    let t4528 = t135 * t1599;
    let t4529 = t973 * t4528;
    let t4531 = t2987 * t1597;
    let t4546 = t974 * t340;
    let t4557 = t1604 * t225;
    (t4514, t4518, t4529, t4531, t4546, t4557)
}
