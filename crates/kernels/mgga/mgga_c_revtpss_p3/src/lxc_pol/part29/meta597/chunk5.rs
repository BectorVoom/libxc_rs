//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2020/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2020<F: Float>(t7407: F, t99272: F, t26482: F, t99404: F, t98849: F, t103363: F, t25305: F, t14991: F, t95936: F, t99373: F, t2435: F, t28390: F) -> (F, F, F, F, F, F, F) {
    let t103382 = F::cast_from(0.14456046980341999104e-1_f64) * t99272 * t7407;
    let t103391 = F::cast_from(0.14456046980341999104e-1_f64) * t99404 * t26482;
    let t103393 = F::cast_from(0.25702851531048074406e-1_f64) * t98849 * t26482;
    let t103394 = t25305 * t103363;
    let t103396 = t95936 * t14991;
    let t103399 = F::cast_from(0.25702851531048074406e-1_f64) * t99373 * t7407;
    let t103400 = t2435 * t28390;
    (t103382, t103391, t103393, t103394, t103396, t103399, t103400)
}
