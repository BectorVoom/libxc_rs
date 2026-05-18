//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 762/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk762<F: Float>(t1080: F, t2475: F, t2844: F, t89: F, t740: F, t113: F, t9494: F, t1068: F, t829: F, t1071: F, t160: F, t239: F) -> (F, F, F, F, F, F, F, F) {
    let t10033 = t2475 * t1080;
    let t10093 = t89 * t2844;
    let t10096 = t740 * t2844;
    let t10099 = t113 * t9494;
    let t10102 = t1068 * t829;
    let t10108 = t160 * t1071;
    let t10109 = t10108 * t829;
    let t10112 = t160 * t239;
    (t10033, t10093, t10096, t10099, t10102, t10108, t10109, t10112)
}
