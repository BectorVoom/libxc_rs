//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1056/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1056<F: Float>(t17793: F, t17795: F, t17797: F, t17800: F, t17803: F, t17805: F, t17809: F, t17811: F, t17814: F, t17817: F, t17819: F, t17822: F, t17826: F, t17828: F, t17831: F, t17833: F, t17835: F, t17837: F) -> (F,) {
    let t18863 = -0.33333333333333333334e0 * t17793 - 0.26979166666666666666e-1 * t17795 + 0.625e-1 * t17797 + 0.10791666666666666667e0 * t17800 + 0.23981481481481481481e-1 * t17803 - 0.9375e-1 * t17805 + 0.89930555555555555554e-2 * t17809 + 0.20234375e-1 * t17811 + 0.55555555555555555557e-1 * t17814 + 0.20234375e-1 * t17817 - 0.13489583333333333333e-1 * t17819 - 0.20234375e-1 * t17822 + 0.101171875e-1 * t17826 - 0.1875e0 * t17828 - 0.5e0 * t17831 - 0.10791666666666666667e0 * t17833 + 0.625e-1 * t17835 - 0.44965277777777777777e-2 * t17837;
    (t18863,)
}
