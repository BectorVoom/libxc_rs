//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 747/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk747<F: Float>(t4426: F, t7038: F, t25245: F, t4430: F, t1561: F, t25266: F, t25270: F, t4462: F, t4447: F, t4452: F, t1945: F, t4371: F, t807: F, t25220: F, t25232: F, t25246: F, t25256: F, t25267: F, t27222: F, t27224: F) -> (F,) {
    let t27226 = t7038 * t4426;
    let t27228 = t25245 * t4430;
    let t27230 = t25266 * t1561;
    let t27232 = t25270 * t4462;
    let t27234 = t25270 * t4447;
    let t27236 = t25270 * t4452;
    let t27239 = t1945 * t4371;
    let t27240 = t807 * t27239;
    let t27242 = -0.25410001404642664113e-4 * t25246 + 0.20007875121765877254e-2 * t25267 + t27222 / 16.0 + 0.85748036236139473945e-2 * t27224 - 0.42874018118069736972e-3 * t27226 - 0.25410001404642664113e-4 * t27228 + 0.20007875121765877254e-2 * t27230 + 0.17149607247227894789e-2 * t27232 - 0.42874018118069736972e-3 * t27234 + 0.17149607247227894789e-2 * t27236 + t25220 - t25232 + 0.57165357490759649296e-4 * t25256 + 0.57165357490759649296e-4 * t27240;
    (t27242,)
}
