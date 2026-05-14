//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 994/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk994<F: Float>(t13485: F, t6179: F, t3935: F, t13900: F, t2163: F, t1309: F, t20025: F, t6199: F, t13472: F, t1315: F, t20097: F, t20104: F, t20107: F, t20112: F, t20117: F, t20121: F, t3948: F, t3955: F, t6157: F) -> (F, F) {
    let t20124 = t13485 * t6179;
    let t20126 = 0.11993859144118211475e-1 * t3935 * t20124;
    let t20127 = t13900 * t2163;
    let t20128 = t1309 * t20127;
    let t20130 = t6199 * t20025;
    let t20131 = t13472 * t20130;
    let t20134 = 0.35981577432354634426e-1 * t20097 * t1315 + 0.17990788716177317213e-1 * t6157 * t3948 + 0.23987718288236422951e-1 * t6157 * t3955 - 0.35981577432354634426e-1 * t3935 * t20104 + 0.23987718288236422951e-1 * t3935 * t20107 + 0.55971342672551653552e-1 * t3935 * t20112 + 0.95950873152945691803e-1 * t3935 * t20117 + 0.35981577432354634426e-1 * t3935 * t20121 - t20126 - 0.39979530480394038251e-2 * t20128 + 0.71963154864709268853e-1 * t3935 * t20131;
    (t20130, t20134)
}
