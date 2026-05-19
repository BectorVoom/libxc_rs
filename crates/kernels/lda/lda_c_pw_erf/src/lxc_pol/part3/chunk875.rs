//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 875/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk875<F: Float>(t1037: F, t1067: F, t325: F, t333: F, t903: F, t907: F, t935: F, t912: F, t936: F, t1039: F, t1070: F, t38: F) -> (F, F, F, F, F) {
    let t8499 = t1067 * t1037;
    let t8505 = F::cast_from(3.436685857643691_f64) * t325 * t903 * t935 * t907 * t333;
    let t8509 = F::new(0.4274) * t325 * t912 * t333 * t936;
    let t8510 = t1070 * t1039;
    let t8512 = t38 * t38;
    (t8499, t8505, t8509, t8510, t8512)
}
