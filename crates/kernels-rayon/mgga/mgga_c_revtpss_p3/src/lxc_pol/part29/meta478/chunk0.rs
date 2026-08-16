//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1754/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1754(t1561: f64, t25266: f64, t25270: f64, t4462: f64, t4447: f64, t4452: f64, t1945: f64, t4371: f64, t807: f64, t4458: f64, t7025: f64, t1549: f64, t25277: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27230 = t25266 * t1561;
    let t27232 = t25270 * t4462;
    let t27234 = t25270 * t4447;
    let t27236 = t25270 * t4452;
    let t27239 = t1945 * t4371;
    let t27240 = t807 * t27239;
    let t27244 = t7025 * t4458;
    let t27246 = t25277 * t1549;
    (t27230, t27232, t27234, t27236, t27239, t27240, t27244, t27246)
}
