//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1189/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1189<F: Float>(t10004: F, t9720: F, t10013: F, t4419: F, t2804: F, t18792: F, t2805: F, t1586: F) -> (F, F, F, F, F) {
    let t34462 = t9720 * t10004;
    let t34465 = t4419 * t10013;
    let t34466 = t2804 * t34465;
    let t34468 = t2805 * t18792;
    let t34469 = t1586 * t34468;
    (t34462, t34465, t34466, t34468, t34469)
}
