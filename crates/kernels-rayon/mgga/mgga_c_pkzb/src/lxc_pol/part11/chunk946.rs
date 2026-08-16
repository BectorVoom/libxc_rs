//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 946/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk946(t10323: f64, t2363: f64, t2393: f64, t3880: f64, t410: f64, t10189: f64, t133: f64, t945: f64, t10070: f64, t2970: f64, t10309: f64, t6455: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10324 = t2363 * t10323;
    let t10331 = t2393 * t10323;
    let t10334 = t410 * t3880;
    let t10335 = t2363 * t10334;
    let t10340 = t10189 * t133;
    let t10341 = t10340 * t945;
    let t10344 = t2393 * t10334;
    let t10349 = t2970 * t10070;
    let t10352 = t6455 * t10309;
    (t10324, t10331, t10334, t10335, t10340, t10341, t10344, t10349, t10352)
}
