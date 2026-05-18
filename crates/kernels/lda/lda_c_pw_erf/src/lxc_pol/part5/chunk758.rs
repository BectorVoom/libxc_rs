//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 758/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk758<F: Float>(t2532: F, t3416: F, t2065: F, t2191: F, t1466: F, t1318: F, t4893: F, t833: F, t4892: F, t5334: F, t826: F, t1401: F, t2466: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6952 = F::new(8.0) / F::new(15.0) * t3416 * t2532;
    let t6953 = t2191 * t2065;
    let t6954 = t1466 * t6953;
    let t6956 = F::new(8.0) / F::new(15.0) * t1318 * t6954;
    let t6957 = t4893 * t833;
    let t6958 = t4892 * t6957;
    let t6960 = F::new(8.0) / F::new(15.0) * t1318 * t6958;
    let t6962 = F::new(8.0) / F::new(45.0) * t5334 * t826;
    let t6963 = t1401 * t2466;
    (t6952, t6953, t6954, t6956, t6957, t6958, t6960, t6962, t6963)
}
