//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 550/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk550<F: Float>(t475: F, t9448: F, t9438: F, t2487: F, t203: F, t539: F, t107: F, t569: F, t9127: F, t568: F, t600: F, t1580: F, t1641: F, t3149: F, t3153: F, t3159: F, t3173: F, t3200: F, t3204: F, t4953: F, t541: F, t574: F, t597: F, t9442: F, t9446: F) -> (F, F, F) {
    let t9449 = t9448 * t475;
    let t9450 = t9438 * t9449;
    let t9451 = t2487 * t9450;
    let t9453 = t539 * t203;
    let t9454 = t107 * t9453;
    let t9461 = t569 * t9127;
    let t9462 = t568 * t9461;
    let t9469 = t600 * t9127;
    let t9470 = t568 * t9469;
    let t9475 = F::new(0.7988109573733489516e-2) * t9442 - F::new(0.15976219147466979032e-1) * t9446 + F::new(0.15976219147466979032e-1) * t9451 - F::new(0.7150097990370085334e0) * t3159 * t9454 + F::new(0.23833659967900284446e0) * t3153 * t541 + F::new(0.23833659967900284446e0) * t3149 * t541 - F::new(0.23005755572352449806e1) * t574 * t9462 - F::new(0.69017266717057349418e1) * t4953 * t3200 + F::new(0.23005755572352449806e1) * t1580 * t3204 + F::new(0.23005755572352449806e1) * t597 * t9470 - F::new(0.46011511144704899612e1) * t1641 * t3173;
    (t9451, t9453, t9475)
}
