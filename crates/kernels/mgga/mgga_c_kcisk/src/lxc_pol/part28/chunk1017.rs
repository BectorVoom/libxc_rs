//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1017/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1017<F: Float>(t1224: F, t1697: F, t22484: F, t22488: F, t7084: F, t4836: F, t8518: F, t22506: F, t4840: F, t22501: F, t22592: F, t10934: F, t10937: F, t17379: F, t17382: F, t17505: F, t17506: F, t23460: F, t23463: F, t23466: F, t23469: F, t23472: F) -> (F, F, F, F, F, F, F) {
    let t23475 = t1224 * t1697 * t22484;
    let t23478 = t1224 * t7084 * t22488;
    let t23481 = t1224 * t4836 * t8518;
    let t23484 = t1224 * t4840 * t22506;
    let t23487 = t1224 * t1697 * t22501;
    let t23490 = t1224 * t1697 * t22592;
    let t23492 = -t10934 - 0.41203703703703703703e-2 * t10937 - 0.82407407407407407408e-2 * t17382 + t17505 - t17506 - 0.12361111111111111111e-1 * t17379 + 0.20601851851851851852e-2 * t23460 - 0.10300925925925925926e-1 * t23463 + 0.37083333333333333333e-1 * t23466 + 0.24722222222222222222e-1 * t23469 - 0.61805555555555555557e-2 * t23472 - 0.55625000000000000001e-1 * t23475 - 0.74166666666666666668e-1 * t23478 + 0.30902777777777777778e-2 * t23481 - 0.61805555555555555555e-2 * t23484 + 0.18541666666666666667e-1 * t23487 - 0.92708333333333333333e-2 * t23490;
    (t23475, t23478, t23481, t23484, t23487, t23490, t23492)
}
