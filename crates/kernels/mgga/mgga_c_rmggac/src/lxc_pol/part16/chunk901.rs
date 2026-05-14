//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 901/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk901<F: Float>(t8497: F, t8577: F, t1734: F, t236: F, t3351: F, t498: F, t7248: F, t6415: F, t9188: F, t3352: F, t511: F, t6418: F, t1704: F, t503: F, t681: F, t1971: F, t495: F, t8517: F, t9969: F) -> (F, F, F, F, F, F) {
    let t47572 = t8577 * t8497;
    let t47577 = t3351 * t7248 * t236 * t1734 * t498;
    let t47581 = t3351 * t9188 * t236 * t6415;
    let t47585 = t3351 * t3352 * t511 * t6418;
    let t47587 = t503 * t1704;
    let t47588 = t47587 * t681;
    let t47594 = t8517 * t1971 * t511 * t9969 * t495;
    (t47572, t47577, t47581, t47585, t47588, t47594)
}
