//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1365/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1365<F: Float>(t119753: F, t119755: F, t119758: F, t119760: F, t119762: F, t119764: F, t119766: F, t119768: F, t119771: F, t119773: F, t119775: F, t119777: F, t119779: F, t119781: F, t119783: F, t119785: F, t119787: F, t119789: F, t119791: F) -> (F,) {
    let t119946 = 0.53958333333333333333e-1 * t119753 - 0.125e0 * t119755 - 0.809375e-1 * t119758 - 0.10791666666666666667e0 * t119760 - 0.33333333333333333333e0 * t119762 - 0.625e-1 * t119764 + 0.25e0 * t119766 - 0.20234375e-1 * t119768 - 0.17986111111111111111e-1 * t119771 - 0.26979166666666666667e-1 * t119773 + 0.125e0 * t119775 + 0.375e0 * t119777 - 0.28777777777777777778e0 * t119779 - 0.10791666666666666667e0 * t119781 + 0.47962962962962962964e-1 * t119783 - 0.26979166666666666667e-1 * t119785 - 0.20833333333333333333e-1 * t119787 - 0.809375e-1 * t119789 - 0.125e0 * t119791;
    (t119946,)
}
