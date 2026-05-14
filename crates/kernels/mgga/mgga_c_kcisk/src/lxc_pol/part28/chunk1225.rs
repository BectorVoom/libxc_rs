//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1225/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1225<F: Float>(t9736: F, t9991: F, t10000: F, t34207: F, t34210: F, t34213: F, t34223: F, t34243: F, t34278: F, t34280: F, t34594: F, t9728: F, t9748: F, t34432: F, t34460: F, t34487: F, t34511: F, t34532: F, t34557: F, t34585: F) -> (F, F) {
    let t34600 = t9991 * t9736;
    let t34605 = -0.23214722222222222222e-2 * t34207 + 0.19345601851851851852e-2 * t34210 - 0.11607361111111111111e-2 * t34213 + 0.52083333333333333333e-2 * t9991 * t9748 + 0.52083333333333333333e-2 * t9991 * t9728 + 0.20104166666666666667e-2 * t34594 * t9728 - 0.11607361111111111111e-2 * t34223 + 0.52083333333333333333e-2 * t10000 * t9748 - 0.17361111111111111111e-2 * t34600 - 0.46429444444444444443e-2 * t34243 + 0.11607361111111111111e-2 * t34278 - 0.30952962962962962962e-2 * t34280;
    let t34608 = t34432 + t34460 + t34487 + t34511 + t34532 + t34557 + t34585 + t34605;
    (t34600, t34608)
}
