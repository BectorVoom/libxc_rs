//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1835/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1835(t26248: f64, t559: f64, t1358: f64, t7715: f64, t1831: f64, t22783: f64, t5234: f64, t6951: f64, t1369: f64, t22788: f64, t5314: f64, t6952: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26249 = t26248 * t559;
    let t26251 = t7715 * t1358;
    let t26255 = t22783 * t1831;
    let t26257 = t5234 * t6951;
    let t26258 = t26257 * t1369;
    let t26260 = t22788 * t1831;
    let t26262 = t6952 * t5314;
    (t26249, t26251, t26255, t26257, t26258, t26260, t26262)
}
