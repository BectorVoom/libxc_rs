//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1182/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1182(t1089: f64, t3687: f64, t9906: f64, t11945: f64, t9895: f64, t11878: f64, t15805: f64, t1936: f64, t3775: f64, t9980: f64, t10079: f64, t11430: f64, t3363: f64) -> (f64, f64, f64, f64, f64) {
    let t33850 = t9906 * t3687 * t1089;
    let t33852 = t9895 * t11945;
    let t33855 = t15805 * t1936 * t11878;
    let t33857 = t3775 * t9980;
    let t33863 = t3363 * t11430 * t10079;
    (t33850, t33852, t33855, t33857, t33863)
}
