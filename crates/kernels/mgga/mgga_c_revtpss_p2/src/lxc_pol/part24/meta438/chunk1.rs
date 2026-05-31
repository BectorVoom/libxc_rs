//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1394/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1394<F: Float>(t47016: F, t521: F, t583: F, t596: F, t525: F, t9603: F, t527: F, t9615: F, t1340: F, t40165: F, t268: F, t520: F) -> (F, F, F, F, F, F) {
    let t47017 = F::cast_from(1440.0_f64) * t47016;
    let t47019 = t583 * t596 * t521;
    let t47020 = F::cast_from(1920.0_f64) * t47019;
    let t47025 = F::cast_from(1.0_f64) / t525 / t9603;
    let t47040 = F::cast_from(1.0_f64) / t527 / t9615;
    let t47059 = F::cast_from(0.12304822629859687989e5_f64) * t1340 * t40165;
    let t47065 = t520 * t268;
    (t47017, t47020, t47025, t47040, t47059, t47065)
}
