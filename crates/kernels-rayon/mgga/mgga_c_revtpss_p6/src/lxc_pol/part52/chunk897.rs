//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 897/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk897(t25270: f64, t4447: f64, t4452: f64, t1945: f64, t4371: f64, t807: f64, t25220: f64, t25232: f64, t25246: f64, t25256: f64, t25267: f64, t27222: f64, t27224: f64, t27226: f64, t27228: f64, t27230: f64, t27232: f64) -> (f64, f64, f64, f64) {
    let t27234 = t25270 * t4447;
    let t27236 = t25270 * t4452;
    let t27239 = t1945 * t4371;
    let t27240 = t807 * t27239;
    let t27242 = -0.25410001404642664113e-4_f64 * t25246 + 0.20007875121765877254e-2_f64 * t25267 + t27222 / 16.0_f64 + 0.85748036236139473945e-2_f64 * t27224 - 0.42874018118069736972e-3_f64 * t27226 - 0.25410001404642664113e-4_f64 * t27228 + 0.20007875121765877254e-2_f64 * t27230 + 0.17149607247227894789e-2_f64 * t27232 - 0.42874018118069736972e-3_f64 * t27234 + 0.17149607247227894789e-2_f64 * t27236 + t25220 - t25232 + 0.57165357490759649296e-4_f64 * t25256 + 0.57165357490759649296e-4_f64 * t27240;
    (t27234, t27236, t27240, t27242)
}
