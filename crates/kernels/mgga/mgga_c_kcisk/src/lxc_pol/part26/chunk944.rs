//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 944/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk944<F: Float>(t1191: F, t25757: F, t1172: F, t12905: F, t7789: F, t12929: F, t13027: F, t19100: F, t19106: F, t19565: F, t19566: F, t25590: F, t25593: F, t25596: F, t25599: F, t25601: F, t25604: F, t25607: F, t25609: F, t25612: F, t25615: F, t25618: F) -> (F, F, F) {
    let t25758 = t25757 * t1191;
    let t25760 = 1.0 * t1172 * t25758;
    let t25762 = 0.16081824322151104822e2 * t12905 * t7789;
    let t25777 = -t13027 - 0.41203703703703703703e-2 * t12929 - 0.82407407407407407408e-2 * t19100 + t19565 - t19566 + 0.12361111111111111111e-1 * t19106 + 0.20601851851851851852e-2 * t25590 - 0.10300925925925925926e-1 * t25593 + 0.37083333333333333333e-1 * t25596 - 0.24722222222222222222e-1 * t25599 - 0.61805555555555555557e-2 * t25601 - 0.55625000000000000001e-1 * t25604 + 0.74166666666666666668e-1 * t25607 + 0.30902777777777777778e-2 * t25609 - 0.61805555555555555555e-2 * t25612 + 0.18541666666666666667e-1 * t25615 - 0.92708333333333333333e-2 * t25618;
    (t25760, t25762, t25777)
}
