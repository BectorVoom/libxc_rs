//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1395/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1395<F: Float>(t17182: F, t35111: F, t9664: F, t2454: F, t2575: F, t1799: F, t9680: F, t34118: F, t34125: F, t121442: F, t9649: F, t112266: F, t112289: F, t112406: F, t112451: F, t116552: F, t116960: F, t116965: F, t121241: F, t121323: F, t121385: F, t2063: F, t33031: F, t33059: F, t34027: F, t34078: F, t34148: F, t34154: F, t35108: F, t35119: F, t5015: F, t7268: F, t9667: F) -> (F, F) {
    let t122098 = t9664 * t17182 * t35111;
    let t122100 = t2575 * t2454;
    let t122102 = t1799 * t122100 * t9680;
    let t122114 = t34125 * t34118;
    let t122116 = t9649 * t121442;
    let t122133 = -0.69444444444444444447e-2 * t122098 - 0.88437037037037037033e-2 * t122102 - 0.23280625000000000001e-2 * t112451 * t35108 - 0.41666666666666666668e-1 * t9664 * t121241 - 0.8041666666666666667e-2 * t34154 * t34148 - 0.46561250000000000002e-2 * t116552 * t34078 + 0.18518518518518518519e-1 * t121323 * t9667 + 0.61728395061728395063e-2 * t122114 - 0.26805555555555555557e-2 * t122116 + 0.69444444444444444447e-2 * t116960 * t34027 + 0.26805555555555555557e-2 * t116965 * t34027 - 0.77602083333333333337e-3 * t112406 * t121385 + 0.69444444444444444446e-2 * t112266 * t35119 + 0.69444444444444444446e-2 * t112289 * t35119 + 0.69444444444444444446e-2 * t33031 * t5015 * t33059 * t2063 * t7268;
    (t122102, t122133)
}
