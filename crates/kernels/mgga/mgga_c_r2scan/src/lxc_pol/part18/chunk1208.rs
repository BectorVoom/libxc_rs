//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1208/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1208<F: Float>(t39900: F, t39903: F, t39908: F, t39912: F, t39920: F, t41609: F, t43407: F, t43410: F, t43413: F, t43415: F, t43418: F, t43421: F) -> F {
    let t43423 = -t39900 - F::new(0.13972381860938637374e0) * t39903 + t41609 - F::new(0.65854491829355115985e-1) * t39908 - t39912 - F::new(0.86682217400542685632e-1) * t43407 + F::new(0.43663693315433241792e-2) * t43410 + F::new(0.13099107994629972538e-1) * t43413 - F::new(0.87327386630866483584e-2) * t43415 + F::new(0.14282990759302185292e-1) * t39920 + F::new(0.23115257973478049502e0) * t43418 + F::new(0.11557628986739024751e0) * t43421;
    t43423
}
