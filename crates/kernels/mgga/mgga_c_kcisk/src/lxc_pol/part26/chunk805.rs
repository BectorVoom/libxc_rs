//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 805/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk805<F: Float>(t212: F, t916: F, t211: F, t210: F, t3236: F, t3245: F, t1032: F, t2689: F, t1001: F, t3271: F, t982: F, t12652: F, t12654: F, t12656: F, t12660: F, t12665: F, t12667: F, t12669: F, t12672: F, t12675: F, t12678: F) -> (F, F, F, F, F, F) {
    let t12680 = t212 * t916;
    let t12681 = 1.0 / t12680;
    let t12682 = t211 * t12681;
    let t12683 = t210 * t12682;
    let t12685 = t3236 * t3245;
    let t12687 = t1032 * t2689;
    let t12689 = t3271 * t1001;
    let t12690 = t982 * t12689;
    let t12692 = t12652 / 8.0 - 3.0 * t12654 - 3.0 / 4.0 * t12656 + 3.0 / 4.0 * t12660 - 3.0 / 32.0 * t12665 + 3.0 / 16.0 * t12667 - 15.0 / 16.0 * t12669 - 3.0 / 32.0 * t12672 - 3.0 / 8.0 * t12675 - 3.0 / 2.0 * t12678 + 15.0 / 8.0 * t12683 + 3.0 / 2.0 * t12685 + 9.0 / 4.0 * t12687 + 15.0 / 16.0 * t12690;
    (t12681, t12683, t12685, t12687, t12690, t12692)
}
