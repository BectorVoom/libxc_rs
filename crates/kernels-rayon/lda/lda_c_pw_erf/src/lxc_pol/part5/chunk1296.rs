//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1296/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1296(t479: f64, t7856: f64, t10749: f64, t10750: f64, t10755: f64, t10757: f64, t10760: f64, t10766: f64, t10775: f64, t11629: f64, t11631: f64, t11633: f64, t11643: f64, t11644: f64, t11652: f64, t164: f64, t18735: f64, t20661: f64) -> f64 {
    let t23157 = t7856 * t479;
    let t23166 = -0.5670973300165402_f64 * t11629 - 0.00035595929614954216_f64 * t11631 - t10749 - 0.031505407223141116_f64 * t20661 * t164 - 0.031505407223141116_f64 * t23157 - 0.09451622166942335_f64 * t11633 - t11643 + 0.5670973300165402_f64 * t11644 - 0.09451622166942335_f64 * t18735 - 0.031505407223141116_f64 * t10750 - t10755 + 0.1890324433388467_f64 * t10757 + t10760 + 0.2634331482256014_f64 * t11652 - t10766 - 0.005926167098672845_f64 * t10775;
    t23166
}
