//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 117/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk117<F: Float>(t103: F, t235: F, t36: F, t37: F) -> (F, F, F) {
    let t278 = F::new(5.1785) * t37 + F::new(0.905775) * t36 + F::new(0.1100325) * t235 + F::new(0.1241775) * t103;
    let t281 = F::new(1.0) + F::new(29.608749977793437) / t278;
    let t282 = f64::ln(t281);
    (t278, t281, t282)
}
