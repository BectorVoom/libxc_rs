//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1127/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1127<F: Float>(t40840: F, t3366: F, t8355: F, t12005: F, t1338: F, t3678: F, t6755: F, t1348: F, t6767: F, t11561: F, t11863: F, t11864: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40841 = F::new(2.0) / F::new(3.0) * t40840;
    let t40844 = t8355 * t3366;
    let t40845 = F::new(2.0) / F::new(3.0) * t40844;
    let t41028 = t1338 * t12005;
    let t41039 = t6755 * t3678;
    let t41042 = t1348 * t12005;
    let t41047 = t6767 * t3678;
    let t41104 = F::new(5.0) / F::new(8.0) * t11561;
    let t41105 = F::new(2.0) * t11863;
    let t41106 = F::new(2.0) * t11864;
    (t40841, t40845, t41028, t41039, t41042, t41047, t41104, t41105, t41106)
}
