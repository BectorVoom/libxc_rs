//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 439/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk439(t43: f64, t1946: f64, t1947: f64, t1990: f64, t616: f64, t635: f64, t72: f64, t88: f64, t29: f64, t125: f64, t26: f64, t1796: f64) -> (f64, f64, f64, f64) {
    let t44 = 0.135e1_f64 <= t43;
    let t1994 = piecewise3(t44, t1946, -8.0_f64 / 3.0_f64 * t1947 * t88 - 16.0_f64 / 3.0_f64 * t616 * t635 - 8.0_f64 / 3.0_f64 * t72 * t1990);
    let t1995 = t29 * t1994;
    let t1996 = t1995 * t125;
    let t1997 = t26 * t1996;
    let t2002 = -t1796;
    (t1994, t1996, t1997, t2002)
}
