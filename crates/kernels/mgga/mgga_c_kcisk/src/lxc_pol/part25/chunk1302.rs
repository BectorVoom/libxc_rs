//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1302/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1302<F: Float>(t1900: F, t415: F, t6945: F, t32909: F, t34125: F, t16692: F, t9687: F, t17353: F, t34012: F, t33056: F, t112460: F, t112462: F, t116584: F, t116645: F, t116651: F, t116656: F, t116659: F, t116662: F, t116666: F, t15930: F, t33031: F, t7234: F) -> (F, F, F, F) {
    let t116669 = t415 * t6945 * t1900;
    let t116672 = 0.18518518518518518519e-1 * t34125 * t32909;
    let t116674 = t415 * t9687 * t16692;
    let t116676 = t17353 * t34012;
    let t116677 = t33056 * t116676;
    let t116679 = 0.69444444444444444446e-2 * t112460 + 0.69444444444444444446e-2 * t112462 + 0.27777777777777777778e-1 * t33031 * t7234 * t116645 * t15930 + 0.23148148148148148148e-2 * t116651 - 0.40208333333333333334e-2 * t33056 * t116584 - 0.16581944444444444444e-2 * t116656 + 0.11054629629629629629e-2 * t116659 - 0.33163888888888888888e-2 * t116662 - 0.49745833333333333332e-2 * t116666 + 0.33163888888888888888e-2 * t116669 - t116672 + 0.66327777777777777776e-2 * t116674 + 0.89351851851851851853e-3 * t116677;
    (t116669, t116674, t116676, t116679)
}
