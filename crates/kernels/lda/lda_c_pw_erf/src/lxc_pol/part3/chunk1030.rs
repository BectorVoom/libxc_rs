//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1030/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1030<F: Float>(t813: F, t9615: F, t4505: F, t668: F, t4518: F, t1403: F, t3974: F, t5155: F, t6728: F, t3667: F, t573: F, t3868: F, t4506: F, t833: F) -> (F, F, F, F, F) {
    let t12063 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t9615 * t813;
    let t12064 = t4505 * t668;
    let t12065 = t12064 * t4518;
    let t12066 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t12065;
    let t12070 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t3974 * t6728 * t5155 * t1403;
    let t12071 = t573 * t3667;
    let t12075 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t4506 * t12071 * t833 * t3868;
    (t12063, t12064, t12066, t12070, t12075)
}
