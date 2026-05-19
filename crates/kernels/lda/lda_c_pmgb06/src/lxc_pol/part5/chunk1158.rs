//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1158/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1158<F: Float>(t10743: F, t10746: F, t10757: F, t10760: F, t10764: F, t10769: F, t10770: F, t10773: F, t10777: F, t17787: F, t17790: F, t20914: F) -> F {
    let t20915 = F::cast_from(0.21642082724729686_f64) * t10743 + t10746 + F::cast_from(0.3246312408709453_f64) * t10757 + F::cast_from(0.03354522822333102_f64) * t10760 + t10764 + t10769 - F::cast_from(0.09618703433213194_f64) * t10770 - t10773 + t10777 + F::new(4.0) * t17787 + F::new(4.0) * t17790 - t20914;
    t20915
}
