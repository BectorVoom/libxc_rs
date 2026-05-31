//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 631/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk631<F: Float>(t1977: F, t4606: F, t1251: F, t34: F, t817: F, t925: F, t1945: F, t325: F, t1950: F, t1955: F, t1333: F, t462: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4607 = t4606 * t1977;
    let t4632 = t1251 * t34;
    let t4657 = t925 * t817;
    let t4659 = t325 * t1945;
    let t4661 = t325 * t1950;
    let t4662 = F::cast_from(0.002518888888888889_f64) * t4661;
    let t4663 = t4606 * t1955;
    let t4688 = t1333 * t34;
    let t4711 = F::cast_from(4.0_f64) * t462;
    (t4607, t4632, t4657, t4659, t4661, t4662, t4663, t4688, t4711)
}
