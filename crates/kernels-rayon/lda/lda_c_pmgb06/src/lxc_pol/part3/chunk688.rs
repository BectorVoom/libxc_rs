//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 688/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk688(t1227: f64, t123: f64, t1309: f64, t1312: f64, t1323: f64, t1324: f64, t2180: f64, t2821: f64, t295: f64, t312: f64, t315: f64, t317: f64, t329: f64, t342: f64, t346: f64, t3482: f64, t3559: f64, t3659: f64, t384: f64, t388: f64, t3974: f64, t3984: f64, t3987: f64, t3991: f64, t4036: f64, t4038: f64, t4043: f64, t4045: f64, t4049: f64, t4053: f64, t4228: f64, t4231: f64, t4234: f64, t4242: f64, t4245: f64, t4249: f64, t4290: f64, t4327: f64, t61: f64, t73: f64, t76: f64, t77: f64) -> f64 {
    let t4329 = (t2821 + t3482) * t61 + 2.0_f64 * t346 * t1312 * t384 + t346 * t388 * t1309 + 18.0_f64 * t2180 * t76 * t1227 * t342 + t346 * t3659 * t73 + 0.020267214298646783_f64 * t123 * t315 * t3974 * t317 + 3.0_f64 * t329 * t77 * t3559 - 0.0008717022455366076_f64 * t3984 - 0.0017434044910732151_f64 * t3987 - t3991 + t4036 + 6.0_f64 * t4038 * t77 + 2.0_f64 * t346 * t4043 * t4045 - 2.0_f64 * t346 * t4049 * t1324 - t346 * t1323 * t4053 + t4228 * t295 - 9.0_f64 * t4231 * t4234 + t4242 - 5.4655730795145296e-05_f64 * t4245 - t4249 + t4290 * t312 + t4327;
    t4329
}
