//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1189/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1189<F: Float>(t10610: F, t10612: F, t10614: F, t10617: F, t10620: F, t1518: F, t185: F, t2099: F, t3671: F, t822: F, t3846: F, t3965: F, t4479: F) -> (F, F, F, F, F, F, F, F) {
    let t13998 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t10610;
    let t13999 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t10612;
    let t14000 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t10614;
    let t14001 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t10617;
    let t14002 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t10620;
    let t14004 = t185 * t1518 * t2099;
    let t14005 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t14004;
    let t14007 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t822 * t3671;
    let t14010 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3965 * t4479 * t3846;
    (t13998, t13999, t14000, t14001, t14002, t14005, t14007, t14010)
}
