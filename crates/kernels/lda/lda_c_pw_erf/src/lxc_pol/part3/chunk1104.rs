//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1104/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1104<F: Float>(t3775: F, t4738: F, t1124: F, t213: F, t1318: F, t4894: F, t1381: F, t4892: F, t4893: F, t2146: F, t4063: F, t4044: F, t4763: F) -> (F, F, F, F, F) {
    let t12915 = F::new(4.0) / F::new(5.0) * t4738 * t3775;
    let t12916 = t1124 * t213;
    let t12918 = t1318 * t12916 * t4894;
    let t12919 = F::new(4.0) / F::new(3.0) * t12918;
    let t12923 = F::new(4.0) / F::new(5.0) * t1318 * t4892 * t4893 * t1381;
    let t12924 = t2146 * t4063;
    let t12925 = F::new(8.0) / F::new(27.0) * t12924;
    let t12927 = F::new(8.0) / F::new(5.0) * t4763 * t4044;
    (t12915, t12919, t12923, t12925, t12927)
}
