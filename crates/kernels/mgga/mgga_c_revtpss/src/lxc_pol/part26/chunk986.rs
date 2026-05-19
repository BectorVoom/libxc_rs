//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 986/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk986<F: Float>(t1209: F, t3727: F, t460: F, t12295: F, t12292: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F) -> (F, F, F) {
    let t12666 = t1209 * t3727;
    let t12673 = t460 * t3727;
    let t12678 = F::cast_from(0.25925925925925925926e-1_f64) * t12295;
    let t12689 = -t12678 + F::cast_from(0.11111111111111111111e-1_f64) * t12297 + F::cast_from(0.55555555555555555555e-2_f64) * t12299 - F::cast_from(0.16666666666666666667e-1_f64) * t12301 - F::cast_from(0.83333333333333333334e-2_f64) * t12303 + F::cast_from(0.92592592592592592592e-2_f64) * t12307 - F::cast_from(0.33333333333333333333e-1_f64) * t12310 - F::cast_from(0.16666666666666666666e-1_f64) * t12292 + F::cast_from(0.50000000000000000001e-1_f64) * t12314 + F::cast_from(0.50000000000000000001e-1_f64) * t12317 + F::cast_from(0.83333333333333333333e-2_f64) * t12320;
    (t12666, t12673, t12689)
}
