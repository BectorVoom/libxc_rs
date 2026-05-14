//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 963/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk963<F: Float>(t1191: F, t19556: F, t1172: F, t19102: F, t19104: F, t12929: F, t12931: F, t12933: F, t12948: F, t13027: F, t19100: F, t19106: F, t19111: F, t19116: F, t19121: F, t19125: F, t19129: F, t19134: F, t19138: F, t19142: F) -> (F, F) {
    let t19557 = t19556 * t1191;
    let t19559 = 1.0 * t1172 * t19557;
    let t19565 = 0.41203703703703703704e-2 * t19102;
    let t19566 = 0.12361111111111111111e-1 * t19104;
    let t19576 = -t13027 - 0.82407407407407407407e-2 * t12929 + 0.20601851851851851852e-2 * t12933 - 0.61805555555555555556e-2 * t12948 + 0.30902777777777777778e-2 * t12931 - 0.41203703703703703704e-2 * t19100 + t19565 - t19566 + 0.67986111111111111113e-1 * t19106 - 0.10300925925925925926e-1 * t19111 + 0.37083333333333333333e-1 * t19116 - 0.24722222222222222222e-1 * t19121 - 0.61805555555555555555e-2 * t19125 - 0.55625000000000000001e-1 * t19129 + 0.74166666666666666668e-1 * t19134 + 0.18541666666666666667e-1 * t19138 - 0.18541666666666666667e-1 * t19142;
    (t19559, t19576)
}
