//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 319/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk319<F: Float>(t1101: F, t286: F, t1067: F, t1090: F, t1093: F, t1095: F, t1098: F, t248: F, t961: F, t970: F, t972: F, t975: F, t982: F) -> (F, F) {
    let t1103 = F::new(20.0) * t1101 * t286;
    let t1104 = -t961 - t970 - F::new(1.1696447245269292) * t972 - F::new(0.0003662289461201309) * t975 + t982 + t1067 + t248 * t1090 + F::new(2.0) * t1093 - F::new(8.0) * t1095 - t1098 + t1103;
    (t1103, t1104)
}
