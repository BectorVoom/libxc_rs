//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1014/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1014(t11000: f64, t1702: f64, t10974: f64, t10977: f64, t10980: f64, t10984: f64, t10987: f64, t10991: f64, t10993: f64, t10997: f64, t1713: f64, t253: f64, t6347: f64, t6349: f64, t6350: f64, t6352: f64, t6356: f64, t6358: f64, t6363: f64, t6367: f64) -> f64 {
    let t11001 = t1702 * t11000;
    let t11004 = t6347 - t6349 + 1.28_f64 * t6350 - 1.28_f64 * t6352 + t6356 - 1.28_f64 * t6358 + 1.28_f64 * t6363 - t6367 + 1.28_f64 * t10974 - 1.28_f64 * t10977 + 1.28_f64 * t253 * t10980 - 1.28_f64 * t253 * t10984 - 1.28_f64 * t10987 + 1.28_f64 * t10991 - 1.28_f64 * t253 * t10993 + 2.56_f64 * t1713 * t10997 - 1.28_f64 * t253 * t11001;
    t11004
}
