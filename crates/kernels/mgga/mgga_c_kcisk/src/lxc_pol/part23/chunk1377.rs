//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1377/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1377<F: Float>(t32000: F, t33366: F, t5600: F, t113581: F, t9446: F, t32105: F, t9801: F, t109832: F, t109836: F, t109838: F, t109846: F, t110452: F, t110578: F, t110593: F, t110595: F, t19972: F, t2718: F, t32015: F, t32035: F, t32096: F, t33377: F, t33384: F, t33389: F, t9433: F, t9796: F) -> (F, F) {
    let t114341 = t5600 * t32000 * t33366;
    let t114351 = t9446 * t113581;
    let t114361 = t9801 * t32105;
    let t114365 = -0.17687407407407407407e-1 * t114341 + 0.89351851851851851853e-3 * t110578 + 0.29479012345679012345e-2 * t109832 + 0.40208333333333333335e-2 * t110452 * t9796 - 0.20833333333333333334e-1 * t33384 * t32015 - 0.120625e-1 * t33377 * t32035 - 0.23148148148148148149e-2 * t114351 + 0.46296296296296296298e-2 * t110593 + 0.46296296296296296298e-2 * t110595 - 0.41666666666666666668e-1 * t32096 * t33389 - 0.20833333333333333334e-1 * t19972 * t9433 * t2718 + 0.11054629629629629629e-2 * t109836 + 0.23148148148148148149e-2 * t114361 - 0.33163888888888888888e-2 * t109838 - 0.11054629629629629629e-2 * t109846;
    (t114341, t114365)
}
