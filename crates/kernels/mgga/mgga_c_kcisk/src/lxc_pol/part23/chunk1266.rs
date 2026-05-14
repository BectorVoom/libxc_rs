//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1266/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1266<F: Float>(t109579: F, t2737: F, t394: F, t4300: F, t32204: F, t3748: F, t32363: F, t9532: F, t1557: F, t21499: F, t533: F) -> (F, F, F, F, F) {
    let t109580 = t2737 * t109579;
    let t109613 = t4300 * t394;
    let t109617 = t3748 * t32204;
    let t109622 = t32363 * t9532;
    let t109626 = t1557 * t533 * t21499;
    (t109580, t109613, t109617, t109622, t109626)
}
