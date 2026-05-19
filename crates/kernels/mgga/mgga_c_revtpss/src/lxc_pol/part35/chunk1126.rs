//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1126/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1126<F: Float>(t25372: F, t95536: F, t2067: F, t41117: F, t25305: F, t95540: F, t10115: F, t2063: F, t10982: F, t2061: F, t9646: F, t7058: F, t95730: F) -> (F, F, F, F, F, F) {
    let t95822 = t25372 * t95536;
    let t95862 = F::cast_from(0.81814717454467823679e-4_f64) * t41117 * t2067;
    let t95891 = F::cast_from(0.91399340044406952588e-2_f64) * t25305 * t95540;
    let t95893 = F::cast_from(0.11044544084478153697e-3_f64) * t10115 * t2063;
    let t95899 = F::cast_from(0.19637199382202157274e-3_f64) * t9646 * t2061 * t10982;
    let t95914 = F::cast_from(0.22487184191643109717e-1_f64) * t7058 * t95730;
    (t95822, t95862, t95891, t95893, t95899, t95914)
}
