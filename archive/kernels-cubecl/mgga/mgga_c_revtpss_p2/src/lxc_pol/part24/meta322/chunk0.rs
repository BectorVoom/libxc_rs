//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1114/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1114<F: Float>(t22718: F, t38: F, t10389: F, t10398: F, t22671: F, t22688: F, t4227: F, t4232: F, t5825: F, t633: F, t637: F, t77: F) -> (F, F) {
    let t22719 = t38 * t22718;
    let t22738 = -F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t10389 * t22688 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t4227 * t5825 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t633 * t22671 + F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t10398 * t22688 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t4232 * t5825 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t637 * t22671;
    let t22739 = t77 * t22738;
    (t22719, t22739)
}
