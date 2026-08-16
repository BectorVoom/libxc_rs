//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 944/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk944(t19871: f64, t3805: f64, t6394: f64, t19956: f64, t550: f64, t6347: f64, t5249: f64, t1799: f64, t3792: f64, t6414: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20454 = t3805 * t19871 * t6394;
    let t20460 = t3805 * t19956 * t6394;
    let t20463 = t550 * t6347;
    let t20465 = t3805 * t5249 * t20463;
    let t20468 = t3792 * t1799;
    let t20470 = t3805 * t19871 * t20468;
    let t20473 = t3792 * t6414;
    (t20454, t20460, t20463, t20465, t20468, t20470, t20473)
}
