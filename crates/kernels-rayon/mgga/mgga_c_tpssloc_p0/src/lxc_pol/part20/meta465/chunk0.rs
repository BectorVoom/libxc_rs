//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1931/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1931(t1653: f64, t3509: f64, t3578: f64, t3516: f64, t1742: f64, t478: f64, t3068: f64, t1244: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15559 = t1653 * t3509;
    let t15560 = t3578 * t15559;
    let t15563 = t1653 * t3516;
    let t15564 = t3578 * t15563;
    let t15567 = t478 * t1742;
    let t15568 = t15567 * t3068;
    let t15569 = t1244 * t15568;
    (t15559, t15560, t15563, t15564, t15567, t15568, t15569)
}
