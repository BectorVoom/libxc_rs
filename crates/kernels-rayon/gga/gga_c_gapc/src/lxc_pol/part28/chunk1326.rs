//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1326/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1326(t23624: f64, t35813: f64, t6181: f64, t2152: f64, t2208: f64, t3649: f64, t3739: f64, t10226: f64, t11640: f64, t828: f64, t10230: f64, t11633: f64) -> (f64, f64, f64, f64, f64) {
    let t35915 = t35813 * t6181 * t23624;
    let t35919 = t3649 * t2152 * t2208 * t3739;
    let t35921 = t10226 * t3739;
    let t35923 = t828 * t11640;
    let t35925 = t10230 * t11633;
    (t35915, t35919, t35921, t35923, t35925)
}
