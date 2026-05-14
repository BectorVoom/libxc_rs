//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1064/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1064<F: Float>(t10760: F, t20298: F, t30628: F, t20305: F, t29837: F, t11640: F, t30370: F, t11842: F, t2651: F, t10810: F, t574: F, t9292: F, t39900: F, t39903: F, t39908: F, t39912: F, t39920: F, t41609: F, t43407: F) -> (F,) {
    let t43410 = t20298 * t10760 * t30628;
    let t43413 = t20305 * t10760 * t29837;
    let t43415 = t30370 * t11640;
    let t43418 = t2651 * t11842;
    let t43421 = t574 * t10810 * t9292;
    let t43423 = -t39900 - 0.13972381860938637374e0 * t39903 + t41609 - 0.65854491829355115985e-1 * t39908 - t39912 - 0.86682217400542685632e-1 * t43407 + 0.43663693315433241792e-2 * t43410 + 0.13099107994629972538e-1 * t43413 - 0.87327386630866483584e-2 * t43415 + 0.14282990759302185292e-1 * t39920 + 0.23115257973478049502e0 * t43418 + 0.11557628986739024751e0 * t43421;
    (t43423,)
}
