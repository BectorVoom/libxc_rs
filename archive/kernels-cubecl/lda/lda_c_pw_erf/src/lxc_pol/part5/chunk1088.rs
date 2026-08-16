//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1088/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1088<F: Float>(t142: F, t14480: F, t1733: F, t1808: F, t1872: F, t1881: F, t18835: F, t19866: F, t20143: F, t20144: F, t20174: F, t20211: F, t20227: F, t2645: F, t2676: F, t296: F, t411: F, t452: F, t454: F, t456: F, t6019: F, t6025: F, t6089: F, t6094: F, t6126: F, t6130: F, t7083: F, t7211: F, t7214: F, t776: F, t777: F, t7992: F, t7996: F, t8001: F, t8759: F) -> F {
    let t20230 = -t1881 * t7992 + F::cast_from(18.0_f64) * t6025 * t19866 * t1872 + F::cast_from(3.0_f64) * t1733 * t20144 - F::cast_from(2.0_f64) * t7214 * t2676 + F::cast_from(6.0_f64) * t6089 * t6019 - F::cast_from(2.0_f64) * t2645 * t6130 + F::cast_from(18.0_f64) * t14480 * t6094 - F::cast_from(2.0_f64) * t1881 * t8001 + F::cast_from(18.0_f64) * t6126 * t776 * t6094 - F::cast_from(0.16213771438917426_f64) * t18835 + F::cast_from(6.0_f64) * t7996 * t452 * t456 + F::cast_from(2.0_f64) * t2645 * t7083 + F::cast_from(6.0_f64) * t1808 * t20143 * t411 + t777 * t454 * t7211 * t142 - F::cast_from(0.0002905674151788692_f64) * t20174 + t8759 + (t20211 + t20227) * t296;
    t20230
}
