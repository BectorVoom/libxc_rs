//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 780/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk780(t12568: f64, t716: f64, t1902: f64, t883: f64, t12691: f64, t2464: f64, t825: f64, t12704: f64, t2684: f64, t1645: f64, t7696: f64, t22980: f64, t2615: f64, t9438: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40634 = t12568 * t716;
    let t40820 = t883 * t1902;
    let t41060 = t825 * t2464 * t12691;
    let t41071 = t2684 * t2464 * t12704;
    let t41105 = t1645 * t7696;
    let t41231 = t2615 * t9438 * t22980;
    (t40634, t40820, t41060, t41071, t41105, t41231)
}
