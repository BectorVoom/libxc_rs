//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1262/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1262<F: Float>(t394: F, t4296: F, t3913: F, t485: F, t14293: F, t2726: F, t32362: F, t9535: F, t4350: F, t4374: F, t14612: F, t1588: F, t20160: F, t32442: F, t32439: F, t32049: F, t3748: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109302 = t4296 * t394;
    let t109311 = t485 * t3913;
    let t109321 = t2726 * t14293;
    let t109366 = t32362 * t9535;
    let t109378 = t4374 * t4350;
    let t109390 = t1588 * t14612;
    let t109398 = t20160 * t32442;
    let t109399 = t32439 * t109398;
    let t109417 = t3748 * t32049;
    (t109302, t109311, t109321, t109366, t109378, t109390, t109398, t109399, t109417)
}
