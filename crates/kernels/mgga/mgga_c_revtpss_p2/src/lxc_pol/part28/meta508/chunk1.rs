//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1901/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1901<F: Float>(t25270: F, t4447: F, t4452: F, t1945: F, t4371: F, t807: F, t25220: F, t25232: F, t25246: F, t25256: F, t25267: F, t27222: F, t27224: F, t27226: F, t27228: F, t27230: F, t27232: F) -> (F, F) {
    let t27234 = t25270 * t4447;
    let t27236 = t25270 * t4452;
    let t27239 = t1945 * t4371;
    let t27240 = t807 * t27239;
    let t27242 = -F::cast_from(0.25410001404642664113e-4_f64) * t25246 + F::cast_from(0.20007875121765877254e-2_f64) * t25267 + t27222 / F::new(16.0) + F::cast_from(0.85748036236139473945e-2_f64) * t27224 - F::cast_from(0.42874018118069736972e-3_f64) * t27226 - F::cast_from(0.25410001404642664113e-4_f64) * t27228 + F::cast_from(0.20007875121765877254e-2_f64) * t27230 + F::cast_from(0.17149607247227894789e-2_f64) * t27232 - F::cast_from(0.42874018118069736972e-3_f64) * t27234 + F::cast_from(0.17149607247227894789e-2_f64) * t27236 + t25220 - t25232 + F::cast_from(0.57165357490759649296e-4_f64) * t25256 + F::cast_from(0.57165357490759649296e-4_f64) * t27240;
    (t27239, t27242)
}
