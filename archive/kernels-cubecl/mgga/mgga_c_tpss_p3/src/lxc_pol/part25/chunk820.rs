//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 820/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk820<F: Float>(t1830: F, t645: F, t5545: F, t5555: F, t5548: F, t5553: F, t5560: F) -> (F, F, F, F) {
    let t5820 = t1830 * t645;
    let t5826 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t5545;
    let t5829 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t5555;
    let t5831 = -t5826 - t5548 / F::cast_from(24.0_f64) - t5553 / F::cast_from(768.0_f64) - t5829 - t5560 / F::cast_from(192.0_f64);
    (t5820, t5826, t5829, t5831)
}
