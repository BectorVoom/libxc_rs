//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1922/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1922<F: Float>(t103181: F, t28313: F, t93317: F, t4534: F, t689: F, t7384: F, t213: F, t28340: F, t26544: F, t27213: F, t14983: F, t26497: F) -> (F, F, F, F, F, F) {
    let t103182 = t28313 * t103181;
    let t103184 = F::cast_from(0.15421710918628844644e0_f64) * t93317 * t103182;
    let t103196 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t7384 * t4534;
    let t103212 = t213 * t28340;
    let t103216 = F::cast_from(0.14456046980341999104e-1_f64) * t27213 * t26544;
    let t103219 = F::cast_from(0.19514881078765566038e-1_f64) * t26497 * t14983;
    (t103182, t103184, t103196, t103212, t103216, t103219)
}
