//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1346/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1346<F: Float>(t5752: F, t5757: F, t1464: F, t15808: F, t2012: F, t3734: F, t7258: F, t1014: F, t7105: F, t7108: F, t1489: F, t7257: F) -> (F, F, F, F, F, F) {
    let t22237 = t5752 * t5757;
    let t22238 = t1464 * t22237;
    let t22240 = t15808 * t2012;
    let t22241 = t1464 * t22240;
    let t22243 = t3734 * t7258;
    let t22244 = t1464 * t22243;
    let t22248 = t1014 * t7105;
    let t22250 = t1014 * t7108;
    let t22252 = t7257 * t1489;
    (t22238, t22241, t22244, t22248, t22250, t22252)
}
