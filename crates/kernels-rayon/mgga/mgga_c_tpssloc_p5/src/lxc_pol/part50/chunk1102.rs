//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1102/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1102(t32927: f64, t6784: f64, t1599: f64, t8400: f64, t6800: f64, t7619: f64, t6799: f64, t1948: f64, t7593: f64, t345: f64, t1615: f64, t8391: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32928 = t6784 * t32927;
    let t32931 = t1599 * t8400;
    let t32934 = t7619 * t6800;
    let t32935 = t6799 * t32934;
    let t32938 = t1948 * t7593;
    let t32939 = t345 * t32938;
    let t32943 = t8391 * t1615;
    (t32928, t32931, t32934, t32935, t32938, t32939, t32943)
}
