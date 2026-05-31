//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 645/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk645<F: Float>(t1308: F, t3828: F, t571: F, t1485: F, t581: F, t1352: F, t593: F, t1356: F, t549: F, t1319: F, t1318: F, t3619: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3829 = t1308 * t3828;
    let t3831 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t571 * t3829;
    let t3832 = t1485 * t581;
    let t3833 = t1352 * t593;
    let t3834 = t3832 * t3833;
    let t3836 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t571 * t3834;
    let t3837 = t1356 * t549;
    let t3838 = t1319 * t3837;
    let t3840 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1318 * t3838;
    let t3841 = t1319 * t3619;
    (t3829, t3831, t3832, t3833, t3834, t3836, t3837, t3838, t3840, t3841)
}
