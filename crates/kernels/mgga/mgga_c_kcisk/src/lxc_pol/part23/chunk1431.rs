//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1431/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1431<F: Float>(t109627: F, t2075: F, t32441: F, t1588: F, t1597: F, t6174: F, t109626: F, t109633: F, t109756: F, t109921: F, t109932: F, t109934: F, t109941: F, t114462: F, t114464: F, t114470: F, t114473: F, t114477: F, t114482: F, t21502: F, t2740: F, t32339: F, t32394: F, t33911: F, t33925: F, t9850: F) -> (F, F, F) {
    let t115722 = t109627 * t2075 * t32441;
    let t115725 = t1588 * t1597;
    let t115726 = t6174 * t115725;
    let t115745 = 0.17024129629629629629e-1 * t114462 - 0.38691203703703703703e-3 * t114464 - 0.13402777777777777778e-2 * t109633 * t115722 + 0.46296296296296296296e-2 * t109626 * t115726 * t21502 - 0.3574074074074074074e-2 * t109756 * t33911 + 0.12345679012345679012e-1 * t32339 * t33925 - 0.50925925925925925926e-1 * t9850 * t32394 * t2740 + 0.92592592592592592592e-2 * t109921 + 0.23214722222222222222e-2 * t114470 + 0.61905925925925925926e-2 * t114473 + 0.38691203703703703703e-3 * t114477 - 0.30952962962962962962e-2 * t114482 - 0.11574074074074074074e-2 * t109932 - 0.34722222222222222222e-2 * t109934 + 0.3086419753086419753e-2 * t109941;
    (t115722, t115725, t115745)
}
