//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1264/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1264<F: Float>(t1299: F, t1500: F, t32388: F, t9524: F, t21559: F, t25: F, t32467: F, t32439: F, t32353: F, t9515: F) -> (F, F, F, F, F, F) {
    let t109504 = t1500 * t1299;
    let t109508 = t9524 * t32388;
    let t109514 = t25 * t21559;
    let t109515 = t109514 * t32467;
    let t109516 = t32439 * t109515;
    let t109518 = t9515 * t32353;
    (t109504, t109508, t109514, t109515, t109516, t109518)
}
