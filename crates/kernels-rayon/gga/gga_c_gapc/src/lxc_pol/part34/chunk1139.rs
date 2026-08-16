//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1139/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1139(t15542: f64, t33287: f64, t7953: f64, t21801: f64, t7259: f64, t7325: f64, t11799: f64, t129: f64, t18866: f64, t11798: f64, t28370: f64, t7453: f64) -> (f64, f64, f64, f64, f64) {
    let t33289 = t7953 * t33287 * t15542;
    let t33291 = t7259 * t21801;
    let t33292 = t33291 * t7325;
    let t33295 = t18866 * t129 * t11799;
    let t33298 = t11798 * t28370 * t7453;
    (t33289, t33291, t33292, t33295, t33298)
}
