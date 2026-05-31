//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1034/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1034<F: Float>(t1245: F, t3966: F, t4495: F, t940: F, t4488: F, t4487: F, t668: F, t4502: F, t3675: F, t521: F, t3807: F, t806: F) -> (F, F, F, F, F) {
    let t12113 = t3966 * t1245;
    let t12114 = t4495 * t940;
    let t12117 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t4488 * t12113 * t12114;
    let t12118 = t4487 * t668;
    let t12119 = t12118 * t4502;
    let t12120 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t12119;
    let t12121 = t521 * t3675;
    let t12125 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t4488 * t12121 * t806 * t3807;
    (t12114, t12117, t12118, t12120, t12125)
}
