//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1416/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1416<F: Float>(t1596: F, t33781: F, t6204: F, t6587: F, t32401: F, t33778: F, t109617: F, t109622: F, t32354: F, t32363: F, t32366: F, t32385: F, t32498: F, t33762: F, t33767: F, t33784: F, t33823: F, t33928: F, t33937: F, t33961: F, t9519: F, t9529: F, t9544: F, t9860: F, t9869: F) -> (F, F) {
    let t115304 = t6204 * t33781 * t6587 * t1596;
    let t115312 = 0.13402777777777777778e-2 * t33778 * t32401;
    let t115333 = -0.23280625e-2 * t33937 * t115304 - 0.10416666666666666667e-1 * t32354 * t33762 - 0.20833333333333333334e-1 * t32354 * t33784 + t115312 + 0.10416666666666666667e-1 * t33928 * t9544 + 0.10416666666666666667e-1 * t33961 * t9544 + 0.52083333333333333333e-2 * t9860 * t32498 + 0.10416666666666666667e-1 * t33928 * t9519 + 0.10416666666666666667e-1 * t33961 * t9519 + 0.15476481481481481481e-2 * t109617 - 0.17361111111111111111e-2 * t109622 + 0.20104166666666666667e-2 * t33767 * t32385 + 0.52083333333333333333e-2 * t32363 * t9869 + 0.10416666666666666667e-1 * t32366 * t9869 - 0.27777777777777777778e-1 * t9529 * t33823;
    (t115304, t115333)
}
