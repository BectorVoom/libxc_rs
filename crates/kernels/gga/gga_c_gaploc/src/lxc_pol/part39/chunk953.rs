//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 953/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk953<F: Float>(t12806: F, t1562: F, t4614: F, t10533: F, t20796: F, t41738: F, t26435: F, t6710: F, t9438: F, t9060: F, t986: F, t1415: F, t1646: F) -> (F, F, F, F) {
    let t42395 = F::new(0.82820720060468819301e2) * t1562 * t4614 * t12806;
    let t42398 = F::new(0.27606906686822939767e2) * t20796 * t10533 * t41738;
    let t42400 = t6710 * t9438 * t26435;
    let t42401 = F::new(0.15976219147466979032e-1) * t42400;
    let t42402 = t9060 * t986;
    let t42405 = F::new(0.35750489951850426669e0) * t1415 * t42402 * t1646;
    (t42395, t42398, t42401, t42405)
}
