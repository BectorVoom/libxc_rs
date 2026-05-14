//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1145/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1145<F: Float>(t2807: F, t32999: F, t33008: F, t33019: F, t33029: F, t33042: F, t33046: F, t33050: F, t33278: F, t33279: F, t33284: F, t33287: F, t33291: F, t33297: F, t9743: F, t33211: F, t33245: F, t33272: F) -> (F,) {
    let t33300 = 0.15476481481481481481e-2 * t32999 + 0.23214722222222222222e-2 * t33008 - 0.23214722222222222222e-2 * t33019 + t33278 - 0.34722222222222222222e-2 * t33279 - 0.23214722222222222222e-2 * t33029 - 0.52083333333333333333e-2 * t33284 * t2807 - 0.10416666666666666667e-1 * t33287 * t2807 - 0.52083333333333333333e-2 * t33291 * t2807 + 0.11607361111111111111e-2 * t33042 + 0.19345601851851851852e-2 * t33046 - 0.23214722222222222222e-2 * t33050 - 0.34722222222222222222e-2 * t33297 * t9743;
    let t33302 = t33211 + t33245 + t33272 + t33300;
    (t33302,)
}
