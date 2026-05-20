//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3090/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3090<F: Float>(t1131: F, t1150: F, t81403: F, t81418: F, t81437: F, t81472: F, t81485: F, t81506: F, t81538: F, t81552: F, t24327: F, t44012: F) -> (F, F) {
    let t81558 = F::new(1.0) * t1131 * (t81403 + t81418 + t81437 + t81472 + t81485 + t81506 + t81538 + t81552) * t1150;
    let t81560 = F::cast_from(0.51726012919273400301e3_f64) * t44012 * t24327;
    (t81558, t81560)
}
