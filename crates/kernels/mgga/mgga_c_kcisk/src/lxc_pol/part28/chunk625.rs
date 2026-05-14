//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 625/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk625<F: Float>(t7156: F, t7157: F, t4838: F, t4891: F, t4936: F, t4943: F, t7076: F, t7079: F, t7082: F, t7086: F, t7100: F, t7108: F, t7116: F, t7118: F, t7122: F, t7125: F, t7128: F, t7132: F) -> (F, F) {
    let t7158 = t7156 * t7157;
    let t7175 = -0.1294625e1 * t7100 + 0.258925e1 * t7108 + t4936 + 0.10064166666666666667e0 * t4838 + 0.10064166666666666667e0 * t7076 - 0.20128333333333333333e0 * t7079 + 0.60385e0 * t7082 + 0.60385e0 * t7086 + 0.82524375e-1 * t7116 + 0.16504875e0 * t7118 + t4943 + 0.11038e0 * t4891 + 0.11038e0 * t7122 - 0.5519e-1 * t7125 + 0.33114e0 * t7128 + 0.33114e0 * t7132;
    (t7158, t7175)
}
