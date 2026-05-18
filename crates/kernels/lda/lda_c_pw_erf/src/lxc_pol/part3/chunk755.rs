//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 755/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk755<F: Float>(t4880: F, t494: F, t1440: F, t1325: F, t1390: F, t2098: F, t542: F, t519: F, t1476: F, t2146: F, t213: F, t473: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4881 = t4880 * t494;
    let t4882 = t1440 * t4881;
    let t4884 = F::new(8.0) / F::new(15.0) * t1325 * t4882;
    let t4885 = t1390 * t2098;
    let t4886 = t4885 * t542;
    let t4887 = t1440 * t4886;
    let t4889 = F::new(8.0) / F::new(15.0) * t519 * t4887;
    let t4891 = F::new(16.0) / F::new(135.0) * t2146 * t1476;
    let t4892 = t473 * t213;
    (t4881, t4882, t4884, t4885, t4886, t4887, t4889, t4891, t4892)
}
