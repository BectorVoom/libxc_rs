//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1223/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1223<F: Float>(t19940: F, t28029: F, t1808: F, t26929: F, t5182: F, t6720: F, t92515: F, t1189: F, t18459: F, t26933: F, t6693: F, t28059: F, t5091: F) -> (F, F, F, F, F, F) {
    let t99945 = t28029 * t19940;
    let t99948 = t1808 * t26929 * t5182;
    let t99950 = t92515 * t6720;
    let t99952 = t18459 * t1189;
    let t99954 = t26933 * t6693;
    let t99956 = t28059 * t5091;
    (t99945, t99948, t99950, t99952, t99954, t99956)
}
