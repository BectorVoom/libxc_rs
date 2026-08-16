//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 663/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk663(t352: f64, t593: f64, t3976: f64, t549: f64, t3974: f64, t2017: f64, t3610: f64, t571: f64, t1410: f64, t640: f64, t653: f64, t256: f64, t3929: f64, t3935: f64, t3938: f64, t3940: f64, t3944: f64, t3947: f64, t3950: f64, t3951: f64, t3955: f64, t3957: f64, t3959: f64, t3960: f64, t3963: f64, t3972: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3977 = t352 * t593;
    let t3979 = t3976 * t3977 * t549;
    let t3981 = 16.0_f64 / 15.0_f64 * t3974 * t3979;
    let t3982 = t2017 * t3610;
    let t3984 = 4.0_f64 / 9.0_f64 * t571 * t3982;
    let t3985 = t640 * t1410;
    let t3988 = 2.0_f64 / 9.0_f64 * t653 * t1410;
    let t3989 = t3929 + t3935 - t3938 + t3940 * t256 / 3.0_f64 + t3944 + 0.18233333333333332_f64 * t3947 + t3950 + 0.36466666666666664_f64 * t3951 + t3955 + t3957 - t3959 + 0.09973633333333333_f64 * t3960 + t3963 - t3972 - t3981 + t3984 - 2.0_f64 / 9.0_f64 * t3985 - t3988;
    (t3977, t3979, t3981, t3982, t3984, t3985, t3988, t3989)
}
