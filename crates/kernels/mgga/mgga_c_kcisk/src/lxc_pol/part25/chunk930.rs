//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 930/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk930<F: Float>(t16458: F, t600: F, t45: F, t6847: F, t15991: F, t10572: F, t10574: F, t10576: F, t15993: F, t16001: F, t16006: F, t16011: F, t16015: F, t16019: F, t16024: F, t16398: F) -> (F, F, F, F) {
    let t16459 = t16458 * t600;
    let t16462 = t45 * t6847;
    let t16485 = 0.13418888888888888889e0 * t15991;
    let t16493 = 0.67094444444444444447e-1 * t10572 - 0.20128333333333333334e0 * t10574 + 0.10064166666666666667e0 * t10576 + t16485 - 0.40256666666666666667e0 * t15993 - 0.33547222222222222222e0 * t16001 + 0.12077e1 * t16006 + 0.80513333333333333333e0 * t16011 - 0.20128333333333333333e0 * t16015 - 0.181155e1 * t16019 - 0.24154e1 * t16024;
    let t16500 = 0.22076e0 * t16398;
    (t16459, t16462, t16493, t16500)
}
