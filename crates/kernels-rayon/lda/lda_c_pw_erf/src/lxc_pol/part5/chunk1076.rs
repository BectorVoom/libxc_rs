//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1076/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1076(t8528: f64, t14445: f64, t405: f64, t7976: f64, t2765: f64, t7970: f64, t10832: f64, t11499: f64, t11501: f64, t14491: f64, t1735: f64, t1808: f64, t1809: f64, t1859: f64, t18795: f64, t19425: f64, t19832: f64, t2211: f64, t2591: f64, t2610: f64, t440: f64, t5495: f64, t5783: f64, t5924: f64, t6025: f64, t6121: f64, t6154: f64, t7082: f64, t770: f64, t777: f64, t7880: f64, t7889: f64, t7986: f64, t7987: f64, t8751: f64, t9156: f64) -> (f64, f64, f64) {
    let t20097 = 60.0_f64 * t8528;
    let t20098 = 1.7544670192365612_f64 * t14445;
    let t20106 = t405 * t7976;
    let t20120 = t2765 * t7970;
    let t20138 = -6.0_f64 * t19832 * t2765 * t7986 * t440 + 36.0_f64 * t6025 * t19425 + 0.11974234010254609_f64 * t18795 + 3.0_f64 * t20106 * t1735 + 9.0_f64 * t2211 * t2591 * t5495 + 9.0_f64 * t2211 * t7082 * t1809 - 9.0_f64 * t5783 * t10832 * t7880 - t11499 - 0.002615106736609823_f64 * t11501 - 0.9247854820715865_f64 * t8751 + 18.0_f64 * t5924 * t20120 + 18.0_f64 * t14491 * t7889 + 18.0_f64 * t1808 * t5495 * t2610 + 18.0_f64 * t1808 * t1809 * t6121 + 2.0_f64 * t777 * t9156 * t7987 + 4.0_f64 * t6154 * t2765 * t770 * t1859;
    (t20097, t20098, t20138)
}
