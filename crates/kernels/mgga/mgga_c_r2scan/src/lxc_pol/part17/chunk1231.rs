//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1231/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1231<F: Float>(t37933: F, t43313: F, t43316: F, t43319: F, t43322: F, t43324: F, t43327: F, t43330: F, t43332: F, t43335: F, t43337: F, t43340: F) -> F {
    let t44343 = -F::new(0.42683466926433871473e0) * t37933 + F::new(0.23115257973478049502e0) * t43313 + F::new(0.27944763721877274748e0) * t43316 + F::new(0.26198215989259945076e-1) * t43319 + F::new(0.93149212406257582492e-1) * t43322 + F::new(0.10401866088065122276e1) * t43324 + F::new(0.87327386630866483588e-2) * t43327 - F::new(0.86682217400542685632e-1) * t43330 - F::new(0.5200933044032561138e0) * t43332 + F::new(0.43663693315433241794e-2) * t43335 + F::new(0.13869154784086829701e1) * t43337 + F::new(0.52396431978519890152e-1) * t43340;
    t44343
}
