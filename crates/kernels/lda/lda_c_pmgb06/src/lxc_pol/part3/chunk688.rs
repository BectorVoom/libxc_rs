//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 688/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk688<F: Float>(t1227: F, t123: F, t1309: F, t1312: F, t1323: F, t1324: F, t2180: F, t2821: F, t295: F, t312: F, t315: F, t317: F, t329: F, t342: F, t346: F, t3482: F, t3559: F, t3659: F, t384: F, t388: F, t3974: F, t3984: F, t3987: F, t3991: F, t4036: F, t4038: F, t4043: F, t4045: F, t4049: F, t4053: F, t4228: F, t4231: F, t4234: F, t4242: F, t4245: F, t4249: F, t4290: F, t4327: F, t61: F, t73: F, t76: F, t77: F) -> F {
    let t4329 = (t2821 + t3482) * t61 + F::new(2.0) * t346 * t1312 * t384 + t346 * t388 * t1309 + F::new(18.0) * t2180 * t76 * t1227 * t342 + t346 * t3659 * t73 + F::cast_from(0.020267214298646783_f64) * t123 * t315 * t3974 * t317 + F::new(3.0) * t329 * t77 * t3559 - F::cast_from(0.0008717022455366076_f64) * t3984 - F::cast_from(0.0017434044910732151_f64) * t3987 - t3991 + t4036 + F::new(6.0) * t4038 * t77 + F::new(2.0) * t346 * t4043 * t4045 - F::new(2.0) * t346 * t4049 * t1324 - t346 * t1323 * t4053 + t4228 * t295 - F::new(9.0) * t4231 * t4234 + t4242 - F::cast_from(5.4655730795145296e-05_f64) * t4245 - t4249 + t4290 * t312 + t4327;
    t4329
}
