//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1385/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1385<F: Float>(t123: F, t2734: F, t34939: F, t109832: F, t114302: F, t119238: F, t119254: F, t119257: F, t119261: F, t119272: F, t32436: F, t33762: F, t33784: F, t33794: F, t33832: F, t33837: F, t33941: F, t35004: F, t9539: F) -> (F,) {
    let t120430 = t2734 * t34939 * t123;
    let t120435 = 0.61905925925925925925e-2 * t119238 - 0.51588271604938271603e-3 * t114302 - 0.23214722222222222222e-2 * t119254 - 0.77382407407407407407e-3 * t119257 + 0.34822083333333333332e-2 * t119261 + 0.10317654320987654321e-2 * t109832 - 0.20833333333333333334e-1 * t33794 * t33832 - 0.10416666666666666667e-1 * t33794 * t33837 - 0.10416666666666666667e-1 * t33941 * t33762 + 0.25794135802469135802e-2 * t119272 - 0.20833333333333333334e-1 * t33941 * t33784 - 0.17361111111111111111e-2 * t120430 * t9539 - 0.23148148148148148148e-2 * t32436 * t35004;
    (t120435,)
}
