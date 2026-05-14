//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1353/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1353<F: Float>(t3936: F, t9452: F, t110605: F, t2075: F, t32070: F, t20233: F, t33359: F, t32087: F, t109420: F, t1339: F, t6234: F, t109952: F, t9814: F, t2173: F, t32033: F, t4158: F, t6204: F) -> (F, F, F, F, F, F, F) {
    let t113735 = t3936 * t9452;
    let t113740 = t110605 * t2075 * t32070;
    let t113745 = t20233 * t33359;
    let t113747 = 0.23148148148148148148e-2 * t32087 * t113745;
    let t113749 = t1339 * t109420 * t6234;
    let t113761 = t1339 * t109952 * t9814;
    let t113765 = t6204 * t32033 * t2173 * t4158;
    (t113735, t113740, t113745, t113747, t113749, t113761, t113765)
}
