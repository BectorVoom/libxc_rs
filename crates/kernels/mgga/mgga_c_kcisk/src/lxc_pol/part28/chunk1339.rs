//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1339/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1339<F: Float>(t33167: F, t34412: F, t17182: F, t34423: F, t9740: F, t34428: F, t10005: F, t33276: F, t12261: F, t9994: F, t9725: F, t34474: F, t9736: F, t34477: F, t34593: F, t9732: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t118040 = t34412 * t33167;
    let t118049 = t9740 * t17182 * t34423;
    let t118051 = t17182 * t34428;
    let t118053 = 0.34722222222222222222e-2 * t9740 * t118051;
    let t118064 = t10005 * t33276;
    let t118069 = t12261 * t9994;
    let t118070 = t9725 * t118069;
    let t118091 = 0.34722222222222222222e-2 * t34474 * t9736;
    let t118098 = 0.34722222222222222222e-2 * t34477 * t9736;
    let t118099 = t34593 * t9732;
    (t118040, t118049, t118051, t118053, t118064, t118069, t118070, t118091, t118098, t118099)
}
