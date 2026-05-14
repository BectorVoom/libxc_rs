//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1090/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1090<F: Float>(t11506: F, t39015: F, t2867: F, t3275: F, t38739: F, t11002: F, t1115: F, t2847: F, t3269: F, t39197: F, t39198: F, t3262: F, t3472: F, t40635: F, t1013: F, t1120: F, t11217: F, t11223: F, t12256: F, t12259: F, t1292: F, t1295: F, t1300: F, t19203: F, t2394: F, t2400: F, t3506: F, t3735: F, t38783: F, t38839: F, t6693: F, t829: F, t8398: F, t8409: F, t8412: F, t8415: F) -> (F, F, F, F, F, F) {
    let t41811 = 3.0 / 2.0 * t11506 * t39015;
    let t41814 = t3275 * t38739 * t2867 / 4.0;
    let t41816 = t11002 * t1115 * t2847;
    let t41818 = 5.0 / 8.0 * t3269 * t41816;
    let t41821 = 15.0 / 4.0 * t39197 * t1115 * t39198;
    let t41824 = 15.0 / 8.0 * t3262 * t3472 * t40635;
    let t41854 = -0.768e1 * t6693 * t12256 * t829 - 0.768e1 * t6693 * t12259 * t829 - 0.384e1 * t6693 * t3735 * t1292 - 0.1536e2 * t19203 * t3735 * t1295 - 0.768e1 * t38839 * t2400 - 0.768e1 * t11223 * t8412 - 0.384e1 * t11223 * t8415 - 0.1536e2 * t38783 * t8409 - 0.128e1 * t1300 * t11217 * t1013 - 0.256e1 * t1300 * t3506 * t2394 - 0.128e1 * t1300 * t1120 * t8398;
    (t41811, t41814, t41818, t41821, t41824, t41854)
}
