//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 546/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk546(t124: f64, t9419: f64, t3192: f64, t574: f64, t475: f64, t9193: f64, t1445: f64, t9177: f64, t1457: f64, t9172: f64, t1562: f64, t1572: f64, t1646: f64, t4527: f64, t536: f64, t597: f64, t7007: f64, t895: f64, t9393: f64, t9396: f64, t9399: f64, t9404: f64, t9409: f64, t9413: f64, t9416: f64) -> (f64, f64, f64, f64, f64) {
    let t9420 = t9419 * t124;
    let t9421 = t9420 * t3192;
    let t9422 = t574 * t9421;
    let t9424 = t9193 * t475;
    let t9425 = t1445 * t9424;
    let t9428 = t1445 * t9177;
    let t9431 = t1457 * t9172;
    let t9434 = 0.71500979903700853338e0_f64 * t895 * t7007 + 0.35750489951850426669e0_f64 * t536 * t9393 - 0.35750489951850426669e0_f64 * t9396 * t1646 - 0.35750489951850426669e0_f64 * t9399 * t1646 - 0.69017266717057349418e1_f64 * t1562 * t9404 + 0.27606906686822939767e2_f64 * t4527 * t9409 - 0.46011511144704899612e1_f64 * t574 * t9413 + 0.11502877786176224903e2_f64 * t597 * t9416 + 0.31952438294933958064e-1_f64 * t9422 + 0.43710935587469654631e2_f64 * t597 * t9425 - 0.62115540045351614476e2_f64 * t1562 * t9428 + 0.71500979903700853338e0_f64 * t1572 * t9431;
    (t9420, t9422, t9424, t9431, t9434)
}
