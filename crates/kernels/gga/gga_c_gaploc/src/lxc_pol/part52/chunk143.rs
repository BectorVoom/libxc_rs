//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 143/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk143<F: Float>(t403: F, t406: F, t408: F, t413: F, t90: F, t257: F, t260: F, t266: F, t657: F, t667: F, t670: F, t61: F, t63: F) -> (F, F, F) {
    let t677 = F::cast_from(0.77371026992393176896e-2_f64) * t90 - F::cast_from(0.2499945e-2_f64) * t403 + F::cast_from(0.604634375e-3_f64) * t406 - F::cast_from(0.20417003743104289064e-4_f64) * t408 + F::cast_from(0.20205871875e-5_f64) * t413;
    let t679 = -F::cast_from(0.10636476373080147432e-2_f64) * t90 * t257 - F::cast_from(0.21272952746160294864e-2_f64) * t657 * t667 - t670 * t266 - t260 * t677;
    let t681 = t61 * t63 * t679;
    (t677, t679, t681)
}
