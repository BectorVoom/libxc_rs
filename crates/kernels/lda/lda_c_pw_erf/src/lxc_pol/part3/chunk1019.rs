//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1019/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1019<F: Float>(t5401: F, t568: F, t1284: F, t3437: F, t10436: F, t548: F, t2104: F, t3994: F, t808: F, t2114: F, t4564: F, t4568: F) -> (F, F, F, F, F, F, F) {
    let t11936 = t5401 * t568;
    let t11937 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t11936;
    let t11939 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1284 * t3437;
    let t11940 = t548 * t10436;
    let t11941 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t11940;
    let t11943 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t2104 * t3437;
    let t11945 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t3994 * t808;
    let t11946 = t2114 * t4564;
    let t11947 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t11946;
    let t11948 = t2114 * t4568;
    (t11937, t11939, t11941, t11943, t11945, t11947, t11948)
}
