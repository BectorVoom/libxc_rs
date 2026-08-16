//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1046/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1046(t11179: f64, t11353: f64, t11356: f64, t11363: f64, t11367: f64, t11369: f64, t455: f64, t552: f64, t6739: f64, t6740: f64, t6743: f64, t6764: f64, t6771: f64, t6792: f64, t6793: f64, t6804: f64, t6806: f64, t6811: f64, t6816: f64, t6823: f64, t6827: f64) -> f64 {
    let t11375 = -t6739 + 6.496391258193384_f64 * t6740 - 6.496391258193384_f64 * t6743 - t6764 - t6771 - 1.8805371096875316_f64 * t11353 * t552 - 3.7610742193750633_f64 * t11356 * t455 + t6792 - 7.35994946043302_f64 * t6793 + t6804 - 3.600163427964126_f64 * t6806 + 22.07984838129906_f64 * t6811 + 5.9648145914819635_f64 * t11363 * t11179 + 2.9824072957409817_f64 * t11367 - 2.427516195194328_f64 * t11369 * t455 - 10.80049028389238_f64 * t6816 - 22.07984838129906_f64 * t6823 + 10.80049028389238_f64 * t6827;
    t11375
}
