//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1166/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1166(t10253: f64, t2312: f64, t21154: f64, t2268: f64, t25775: f64, t10160: f64, t1349: f64, t25730: f64, t4261: f64, t9074: f64, t10276: f64, t3808: f64) -> (f64, f64, f64, f64, f64) {
    let t31564 = t2312 * t10253;
    let t31565 = 0.23712505529730124666e-2_f64 * t31564;
    let t31568 = 0.17073003981405689759e1_f64 * t2268 * t25775 * t21154;
    let t31569 = t1349 * t10160;
    let t31570 = 0.31616674039640166222e-2_f64 * t31569;
    let t31574 = t9074 * t4261 * t25730;
    let t31575 = 0.47425011059460249332e-2_f64 * t31574;
    let t31577 = 0.18970004423784099733e-1_f64 * t3808 * t10276;
    (t31565, t31568, t31570, t31575, t31577)
}
