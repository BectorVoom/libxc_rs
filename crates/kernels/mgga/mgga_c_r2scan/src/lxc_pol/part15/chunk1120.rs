//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1120/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1120<F: Float>(t39421: F, t39422: F, t39424: F, t39426: F, t39429: F, t39431: F, t39434: F, t39438: F, t39440: F, t39444: F, t39446: F, t39448: F) -> F {
    let t39450 = -t39421 + F::new(0.54878743191129263322e-1) * t39422 + F::new(0.10975748638225852664e0) * t39424 - F::new(0.27439371595564631661e-1) * t39426 + F::new(0.15573871527278325618e-1) * t39429 + F::new(0.54878743191129263322e-1) * t39431 + F::new(0.86682217400542685632e-1) * t39434 + t39438 - F::new(0.95219938395347901943e-2) * t39440 - t39444 + t39446 - F::new(0.10401866088065122276e1) * t39448;
    t39450
}
