//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 977/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk977<F: Float>(t3818: F, t3965: F, t4483: F, t12109: F, t3398: F, t10011: F, t4480: F, t108: F, t2113: F, t267: F, t10015: F, t13009: F, t13011: F, t13013: F, t13016: F, t13018: F, t13022: F, t13025: F) -> (F, F, F, F, F, F) {
    let t13028 = 8.0 / 15.0 * t3965 * t4483 * t3818;
    let t13031 = 8.0 / 9.0 * t3965 * t12109 * t3398;
    let t13032 = t10011 * t4480;
    let t13033 = 32.0 / 45.0 * t13032;
    let t13035 = t2113 * t108 * t267;
    let t13037 = 16.0 / 15.0 * t13035 * t4480;
    let t13039 = 16.0 / 15.0 * t10015 * t4480;
    let t13040 = t13009 + t13011 - t13013 + t13016 - t13018 - t13022 - t13025 - t13028 - t13031 - t13033 - t13037 - t13039;
    (t13028, t13031, t13033, t13037, t13039, t13040)
}
