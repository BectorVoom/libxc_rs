//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1161/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1161<F: Float>(t27556: F, t28772: F, t94621: F, t94624: F, t95130: F, t98663: F, t98666: F, t98673: F, t98676: F, t98680: F, t98684: F, t99524: F, t12844: F, t27583: F, t28748: F, t27566: F, t28720: F) -> (F, F, F) {
    let t99534 = 0.7722800925925925926e-4 * t95130 + 0.46429444444444444443e-2 * t98663 - 0.15476481481481481481e-2 * t98666 + 0.7722800925925925926e-4 * t99524 - 0.17411041666666666666e-2 * t98673 + 0.61905925925925925924e-2 * t98676 + 0.23214722222222222222e-2 * t98680 + 0.51588271604938271604e-3 * t98684 + 0.92754700520833333334e-4 * t27556 * t28772 - 0.25794135802469135802e-3 * t94621 - 0.23214722222222222222e-2 * t94624;
    let t99556 = 0.7722800925925925926e-4 * t27583 * t12844 * t28748;
    let t99565 = t28720 * t27566;
    (t99534, t99556, t99565)
}
