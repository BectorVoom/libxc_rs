//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 751/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk751<F: Float>(t12868: F, t1580: F, t12806: F, t1562: F, t4614: F, t10533: F, t20796: F, t41738: F, t26435: F, t6710: F, t9438: F, t9060: F, t986: F, t1415: F, t1646: F, t12990: F, t7007: F) -> (F, F, F, F, F, F) {
    let t42392 = 0.11502877786176224903e2 * t1580 * t12868;
    let t42395 = 0.82820720060468819301e2 * t1562 * t4614 * t12806;
    let t42398 = 0.27606906686822939767e2 * t20796 * t10533 * t41738;
    let t42400 = t6710 * t9438 * t26435;
    let t42401 = 0.15976219147466979032e-1 * t42400;
    let t42402 = t9060 * t986;
    let t42405 = 0.35750489951850426669e0 * t1415 * t42402 * t1646;
    let t42407 = 0.71500979903700853338e0 * t12990 * t7007;
    (t42392, t42395, t42398, t42401, t42405, t42407)
}
