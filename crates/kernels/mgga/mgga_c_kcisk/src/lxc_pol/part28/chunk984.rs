//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 984/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk984<F: Float>(t1664: F, t22867: F, t1645: F, t10710: F, t8577: F, t10570: F, t10761: F, t15989: F, t15996: F, t16447: F, t16448: F, t22564: F, t22567: F, t22570: F, t22573: F, t22575: F, t22578: F, t22581: F, t22583: F, t22586: F, t22589: F, t22594: F) -> (F, F, F) {
    let t22868 = t22867 * t1664;
    let t22870 = 1.0 * t1645 * t22868;
    let t22872 = 0.16081824322151104822e2 * t10710 * t8577;
    let t22887 = -t10761 - 0.41203703703703703703e-2 * t10570 - 0.82407407407407407408e-2 * t15989 + t16447 - t16448 - 0.12361111111111111111e-1 * t15996 + 0.20601851851851851852e-2 * t22564 - 0.10300925925925925926e-1 * t22567 + 0.37083333333333333333e-1 * t22570 + 0.24722222222222222222e-1 * t22573 - 0.61805555555555555557e-2 * t22575 - 0.55625000000000000001e-1 * t22578 - 0.74166666666666666668e-1 * t22581 + 0.30902777777777777778e-2 * t22583 - 0.61805555555555555555e-2 * t22586 + 0.18541666666666666667e-1 * t22589 - 0.92708333333333333333e-2 * t22594;
    (t22870, t22872, t22887)
}
