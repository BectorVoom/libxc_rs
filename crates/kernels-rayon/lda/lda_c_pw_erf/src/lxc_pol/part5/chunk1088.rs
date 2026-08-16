//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1088/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1088(t142: f64, t14480: f64, t1733: f64, t1808: f64, t1872: f64, t1881: f64, t18835: f64, t19866: f64, t20143: f64, t20144: f64, t20174: f64, t20211: f64, t20227: f64, t2645: f64, t2676: f64, t296: f64, t411: f64, t452: f64, t454: f64, t456: f64, t6019: f64, t6025: f64, t6089: f64, t6094: f64, t6126: f64, t6130: f64, t7083: f64, t7211: f64, t7214: f64, t776: f64, t777: f64, t7992: f64, t7996: f64, t8001: f64, t8759: f64) -> f64 {
    let t20230 = -t1881 * t7992 + 18.0_f64 * t6025 * t19866 * t1872 + 3.0_f64 * t1733 * t20144 - 2.0_f64 * t7214 * t2676 + 6.0_f64 * t6089 * t6019 - 2.0_f64 * t2645 * t6130 + 18.0_f64 * t14480 * t6094 - 2.0_f64 * t1881 * t8001 + 18.0_f64 * t6126 * t776 * t6094 - 0.16213771438917426_f64 * t18835 + 6.0_f64 * t7996 * t452 * t456 + 2.0_f64 * t2645 * t7083 + 6.0_f64 * t1808 * t20143 * t411 + t777 * t454 * t7211 * t142 - 0.0002905674151788692_f64 * t20174 + t8759 + (t20211 + t20227) * t296;
    t20230
}
