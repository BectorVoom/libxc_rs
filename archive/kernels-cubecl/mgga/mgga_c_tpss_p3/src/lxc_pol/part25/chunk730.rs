//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 730/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk730<F: Float>(t4799: F, t812: F, t1396: F, t253: F, t3695: F, t4779: F, t4784: F, t809: F) -> (F, F) {
    let t4800 = t812 * t4799;
    let t4802 = -F::cast_from(2.0_f64) * t1396 * t3695 + t253 * t4779 + F::cast_from(2.0_f64) * t4784 * t809 - t4800 * t809;
    (t4800, t4802)
}
