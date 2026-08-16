//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 909/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk909<F: Float>(t12194: F, t16950: F, t1930: F, t3967: F, t4007: F, t5726: F, t613: F, t1377: F, t3977: F, t3754: F, t1380: F, t5654: F) -> (F, F, F, F, F, F) {
    let t16951 = t12194 * t16950;
    let t16954 = t1930 * t3967;
    let t16958 = t5726 * t4007;
    let t16959 = t613 * t16958;
    let t16962 = t3977 * t1377;
    let t16963 = t16962 * t3754;
    let t16964 = t5654 * t1380;
    (t16951, t16954, t16959, t16962, t16963, t16964)
}
