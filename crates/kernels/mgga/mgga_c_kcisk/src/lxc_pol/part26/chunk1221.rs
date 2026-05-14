//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1221/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1221<F: Float>(t25: F, t32457: F, t1310: F, t1311: F, t4374: F, t32388: F, t9524: F, t21559: F, t32353: F, t9515: F, t9512: F, t1557: F, t21499: F, t533: F, t32440: F, t3936: F) -> (F, F, F, F, F, F, F, F) {
    let t109494 = t25 * t32457;
    let t109499 = t1310 * t1311 * t4374;
    let t109508 = t9524 * t32388;
    let t109514 = t25 * t21559;
    let t109518 = t9515 * t32353;
    let t109539 = t9512 * t32388;
    let t109626 = t1557 * t533 * t21499;
    let t109627 = t3936 * t32440;
    (t109494, t109499, t109508, t109514, t109518, t109539, t109626, t109627)
}
