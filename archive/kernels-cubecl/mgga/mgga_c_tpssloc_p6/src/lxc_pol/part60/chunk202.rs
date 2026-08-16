//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 202/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk202<F: Float>(t287: F, t275: F, t276: F, t880: F, t273: F, t241: F, t697: F, t281: F, t283: F, t340: F, t290: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t891 = t287 * t287;
    let t892 = F::cast_from(1.0_f64) / t891;
    let t893 = t275 * t892;
    let t894 = F::cast_from(1.0_f64) / t276;
    let t899 = F::cast_from(0.29896666666666666667e0_f64) * t880;
    let t901 = F::sqrt(t273);
    let t904 = t697 * t241;
    let t906 = t281 * t904 * t283;
    let t907 = F::cast_from(0.82156666666666666667e-1_f64) * t906;
    let t908 = t241 * t340;
    let t913 = F::cast_from(1.0_f64) / t290;
    let t917 = F::cast_from(0.17123333333333333333e-1_f64) * t880;
    (t891, t892, t893, t894, t899, t901, t904, t906, t907, t908, t913, t917)
}
