//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1046/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1046<F: Float>(t11179: F, t11353: F, t11356: F, t11363: F, t11367: F, t11369: F, t455: F, t552: F, t6739: F, t6740: F, t6743: F, t6764: F, t6771: F, t6792: F, t6793: F, t6804: F, t6806: F, t6811: F, t6816: F, t6823: F, t6827: F) -> F {
    let t11375 = -t6739 + F::cast_from(6.496391258193384_f64) * t6740 - F::cast_from(6.496391258193384_f64) * t6743 - t6764 - t6771 - F::cast_from(1.8805371096875316_f64) * t11353 * t552 - F::cast_from(3.7610742193750633_f64) * t11356 * t455 + t6792 - F::cast_from(7.35994946043302_f64) * t6793 + t6804 - F::cast_from(3.600163427964126_f64) * t6806 + F::cast_from(22.07984838129906_f64) * t6811 + F::cast_from(5.9648145914819635_f64) * t11363 * t11179 + F::cast_from(2.9824072957409817_f64) * t11367 - F::cast_from(2.427516195194328_f64) * t11369 * t455 - F::cast_from(10.80049028389238_f64) * t6816 - F::cast_from(22.07984838129906_f64) * t6823 + F::cast_from(10.80049028389238_f64) * t6827;
    t11375
}
