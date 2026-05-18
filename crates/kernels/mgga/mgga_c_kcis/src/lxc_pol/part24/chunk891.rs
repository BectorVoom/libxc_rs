//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 891/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk891<F: Float>(t19186: F, t19214: F, t19252: F, t19292: F, t1009: F, t4824: F, t5026: F, t1092: F, t4773: F, t3178: F, t6614: F, t2855: F, t6486: F) -> (F, F, F, F, F, F) {
    let t19294 = t19186 + t19214 + t19252 + t19292;
    let t19295 = t19294 * t1009;
    let t19300 = t5026 * t4824;
    let t19301 = t1092 * t19300;
    let t19303 = t5026 * t4773;
    let t19304 = t1092 * t19303;
    let t19306 = t3178 * t6614;
    let t19307 = t1092 * t19306;
    let t19309 = t2855 * t6486;
    (t19294, t19295, t19301, t19304, t19307, t19309)
}
