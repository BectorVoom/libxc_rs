//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 973/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk973<F: Float>(t604: F, t16562: F, t1783: F, t1310: F, t164: F, t2465: F, t1773: F, t4989: F, t7253: F, t1774: F, t662: F, t695: F, t964: F, t1772: F, t7202: F, t25: F, t7269: F) -> (F, F, F, F, F, F, F) {
    let t659 = 0.0 < t604;
    let t17269 = piecewise3(t659, t16562, -t16562);
    let t17270 = t1783 * t17269;
    let t17271 = t1310 * t17270;
    let t17276 = t164 * t2465;
    let t17277 = t1773 * t17276;
    let t17280 = 0.35981577432354634426e-1 * t4989 * t7253;
    let t17282 = t1774 * t662 * t695;
    let t17283 = t964 * t17282;
    let t17290 = t7202 * t1772;
    let t17293 = t25 * t7269;
    (t17269, t17271, t17277, t17280, t17283, t17290, t17293)
}
