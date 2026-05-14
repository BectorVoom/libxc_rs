//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1055/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1055<F: Float>(t10935: F, t3165: F, t3446: F, t158: F, t3128: F, t3447: F, t874: F, t122: F, t3434: F, t3437: F, t797: F, t8629: F, t481: F, t9573: F, t2847: F, t3582: F) -> (F, F, F, F, F, F) {
    let t43921 = t3446 * t10935 * t3165;
    let t43936 = t158 * t3128;
    let t43939 = t3446 * t3447 * t43936 * t874;
    let t43943 = t3434 * t3437 * t43936 * t122;
    let t43950 = t797 * t8629;
    let t43959 = t9573 * t481;
    let t43979 = t3582 * t2847;
    (t43921, t43939, t43943, t43950, t43959, t43979)
}
