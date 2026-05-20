//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2561/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2561<F: Float>(t527: F, t9615: F, t1340: F, t40165: F, t2626: F, t9551: F, t512: F, t749: F, t9363: F, t268: F, t520: F, t39768: F) -> (F, F, F, F, F, F) {
    let t47040 = F::new(1.0) / t527 / t9615;
    let t47059 = F::cast_from(0.12304822629859687989e5_f64) * t1340 * t40165;
    let t47060 = t9551 * t2626;
    let t47063 = t512 * t9363 * t749;
    let t47065 = t520 * t268;
    let t47067 = F::cast_from(0.19263893255070628431e1_f64) * t47065 * t39768;
    (t47040, t47059, t47060, t47063, t47065, t47067)
}
