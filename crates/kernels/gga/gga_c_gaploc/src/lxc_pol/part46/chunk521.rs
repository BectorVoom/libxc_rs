//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 521/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk521<F: Float>(t124: F, t9419: F, t3192: F, t574: F, t475: F, t9193: F, t1445: F, t9177: F, t1457: F, t9172: F, t1562: F, t1572: F, t1646: F, t4527: F, t536: F, t597: F, t7007: F, t895: F, t9393: F, t9396: F, t9399: F, t9404: F, t9409: F, t9413: F, t9416: F) -> (F, F, F, F, F) {
    let t9420 = t9419 * t124;
    let t9421 = t9420 * t3192;
    let t9422 = t574 * t9421;
    let t9424 = t9193 * t475;
    let t9425 = t1445 * t9424;
    let t9428 = t1445 * t9177;
    let t9431 = t1457 * t9172;
    let t9434 = F::cast_from(0.71500979903700853338e0_f64) * t895 * t7007 + F::cast_from(0.35750489951850426669e0_f64) * t536 * t9393 - F::cast_from(0.35750489951850426669e0_f64) * t9396 * t1646 - F::cast_from(0.35750489951850426669e0_f64) * t9399 * t1646 - F::cast_from(0.69017266717057349418e1_f64) * t1562 * t9404 + F::cast_from(0.27606906686822939767e2_f64) * t4527 * t9409 - F::cast_from(0.46011511144704899612e1_f64) * t574 * t9413 + F::cast_from(0.11502877786176224903e2_f64) * t597 * t9416 + F::cast_from(0.31952438294933958064e-1_f64) * t9422 + F::cast_from(0.43710935587469654631e2_f64) * t597 * t9425 - F::cast_from(0.62115540045351614476e2_f64) * t1562 * t9428 + F::cast_from(0.71500979903700853338e0_f64) * t1572 * t9431;
    (t9420, t9422, t9424, t9431, t9434)
}
