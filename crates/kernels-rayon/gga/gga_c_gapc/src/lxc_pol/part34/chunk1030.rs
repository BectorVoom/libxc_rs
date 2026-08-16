//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1030/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1030(t612: f64, t7953: f64, t291: f64, t7956: f64, t9066: f64, t3363: f64, t3687: f64, t1089: f64, t3368: f64, t3772: f64, t1084: f64, t11473: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11887 = t7953 * t612;
    let t11889 = t9066 * t291 * t7956;
    let t11890 = t11887 * t11889;
    let t11892 = t3363 * t3687;
    let t11893 = t11892 * t1089;
    let t11895 = t3772 * t3368;
    let t11897 = t1084 * t11473;
    (t11887, t11889, t11890, t11892, t11893, t11895, t11897)
}
