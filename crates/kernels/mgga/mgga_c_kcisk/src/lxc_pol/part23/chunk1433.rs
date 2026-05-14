//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1433/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1433<F: Float>(t109627: F, t1163: F, t33835: F, t33870: F, t9529: F, t109518: F, t109633: F, t109956: F, t114552: F, t114555: F, t114558: F, t114566: F, t114573: F, t115423: F, t115482: F, t32353: F, t32371: F, t32376: F, t32395: F, t32439: F, t32485: F, t33762: F, t33784: F, t33937: F, t9855: F, t9860: F, t9869: F) -> (F, F) {
    let t115796 = t109627 * t33835 * t1163;
    let t115806 = t9529 * t33870;
    let t115810 = -0.23280625e-2 * t32376 * t32353 * t33784 + 0.116403125e-2 * t33937 * t115423 - 0.40208333333333333334e-2 * t109518 * t33762 - 0.20104166666666666667e-2 * t32439 * t115482 + 0.15476481481481481481e-2 * t109956 + 0.46429444444444444443e-2 * t114552 + 0.92858888888888888886e-2 * t114555 - 0.13402777777777777778e-2 * t109633 * t115796 - 0.92858888888888888886e-2 * t114558 + 0.52083333333333333333e-2 * t32371 * t9855 - 0.10416666666666666667e-1 * t9860 * t32485 + 0.50925925925925925926e-1 * t32395 * t9869 - 0.92592592592592592593e-2 * t115806 - 0.46429444444444444444e-2 * t114566 + 0.11607361111111111111e-2 * t114573;
    (t115796, t115810)
}
