//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 988/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk988(t142: f64, t5548: f64, t455: f64, t10833: f64, t10843: f64, t11482: f64, t11486: f64, t11495: f64, t11499: f64, t11501: f64, t11507: f64, t1733: f64, t1735: f64, t1881: f64, t2208: f64, t2211: f64, t2806: f64, t4283: f64, t452: f64, t456: f64, t5490: f64, t5783: f64, t776: f64, t8751: f64, t9130: f64) -> (f64, f64) {
    let t11510 = t142 * t5548;
    let t11511 = t455 * t11510;
    let t11516 = 6.0_f64 * t4283 * t776 * t456 - 0.16213771438917426_f64 * t11482 + 3.0_f64 * t10843 * t2208 + 9.0_f64 * t1733 * t11486 - 6.0_f64 * t1881 * t2806 + 9.0_f64 * t2211 * t9130 - 0.0008717022455366076_f64 * t11495 - t11499 - 0.0008717022455366076_f64 * t11501 - 2.7743564462147594_f64 * t8751 + 18.0_f64 * t5490 * t452 * t2208 + 9.0_f64 * t11507 * t1735 + 9.0_f64 * t1733 * t11511 - 18.0_f64 * t5783 * t10833;
    (t11510, t11516)
}
