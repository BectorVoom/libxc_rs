//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 971/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk971<F: Float>(t12916: F, t1318: F, t4894: F, t1381: F, t4892: F, t4893: F, t2146: F, t4063: F, t4044: F, t4763: F, t10031: F, t3977: F, t5155: F, t3974: F, t4515: F, t12900: F, t12902: F, t12903: F, t12907: F, t12909: F, t12913: F, t12915: F) -> (F, F, F, F, F, F, F, F) {
    let t12918 = t1318 * t12916 * t4894;
    let t12919 = 4.0 / 3.0 * t12918;
    let t12923 = 4.0 / 5.0 * t1318 * t4892 * t4893 * t1381;
    let t12924 = t2146 * t4063;
    let t12925 = 8.0 / 27.0 * t12924;
    let t12927 = 8.0 / 5.0 * t4763 * t4044;
    let t12928 = 32.0 / 45.0 * t10031;
    let t12929 = t5155 * t3977;
    let t12932 = 32.0 / 15.0 * t3974 * t4515 * t12929;
    let t12933 = -t12900 + t12902 - t12903 + t12907 + t12909 - t12913 - t12915 - t12919 + t12923 + t12925 + t12927 - t12928 + t12932;
    (t12919, t12923, t12925, t12927, t12928, t12929, t12932, t12933)
}
