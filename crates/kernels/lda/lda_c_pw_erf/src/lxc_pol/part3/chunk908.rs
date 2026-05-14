//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 908/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk908<F: Float>(t11936: F, t1284: F, t3437: F, t10436: F, t548: F, t2104: F, t3994: F, t808: F, t2114: F, t4564: F, t4568: F, t1511: F, t184: F, t1980: F, t199: F, t1529: F, t1960: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11937 = 8.0 / 15.0 * t11936;
    let t11939 = 4.0 / 5.0 * t1284 * t3437;
    let t11940 = t548 * t10436;
    let t11941 = 16.0 / 15.0 * t11940;
    let t11943 = 4.0 / 5.0 * t2104 * t3437;
    let t11945 = 4.0 / 5.0 * t3994 * t808;
    let t11946 = t2114 * t4564;
    let t11947 = 8.0 / 45.0 * t11946;
    let t11948 = t2114 * t4568;
    let t11949 = 4.0 / 3.0 * t11948;
    let t11953 = 4.0 / 5.0 * t1511 * t1980 * t184 * t199;
    let t11954 = t1960 * t1529;
    (t11937, t11939, t11941, t11943, t11945, t11947, t11949, t11953, t11954)
}
