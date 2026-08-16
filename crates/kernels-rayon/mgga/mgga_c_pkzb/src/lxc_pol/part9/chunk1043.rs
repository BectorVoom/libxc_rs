//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1043/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1043(t10257: f64, t3232: f64, t1697: f64, t95: f64, t1143: f64, t799: f64, t1054: f64, t633: f64, t440: f64, t973: f64, t1255: f64, t951: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10258 = t3232 * t10257;
    let t11817 = t1697 * t95;
    let t12315 = t1143 * t799;
    let t12508 = t1054 * t633;
    let t12584 = t973 * t440;
    let t12845 = t1255 * t951;
    (t10258, t11817, t12315, t12508, t12584, t12845)
}
