//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1379/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1379<F: Float>(t1596: F, t33781: F, t6204: F, t8403: F, t20160: F, t34930: F, t9536: F, t109652: F, t109655: F, t115375: F, t119066: F, t119069: F, t119072: F, t119076: F, t119079: F, t119083: F, t32339: F, t32436: F, t33937: F, t34931: F, t35012: F) -> (F, F, F) {
    let t120285 = t6204 * t33781 * t8403 * t1596;
    let t120292 = t20160 * t34930;
    let t120293 = t9536 * t120292;
    let t120301 = t115375 - 0.11574074074074074074e-2 * t109652 - 0.11574074074074074074e-2 * t109655 - 0.116403125e-2 * t33937 * t120285 + 0.34722222222222222222e-2 * t32436 * t35012 + 0.27777777777777777779e-1 * t32339 * t34931 - 0.34722222222222222223e-2 * t120293 + 0.23214722222222222222e-2 * t119066 + 0.46429444444444444444e-2 * t119069 + 0.11607361111111111111e-2 * t119072 + 0.11607361111111111111e-2 * t119076 + 0.19345601851851851852e-2 * t119079 - 0.15476481481481481481e-2 * t119083;
    (t120285, t120292, t120301)
}
