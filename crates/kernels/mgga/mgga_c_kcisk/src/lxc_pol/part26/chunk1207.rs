//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1207/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1207<F: Float>(t9850: F, t9859: F, t27694: F, t79: F, t2736: F, t2740: F, t32502: F, t33778: F, t33966: F, t34781: F, t34784: F, t34787: F, t34790: F, t34807: F, t34811: F, t34817: F, t34827: F, t9855: F) -> (F, F, F, F) {
    let t35018 = t9850 * t9859;
    let t35025 = t27694 * t79;
    let t35026 = t35025 * t2736;
    let t35035 = -0.34822083333333333332e-2 * t34781 - 0.10416666666666666667e-1 * t35018 * t2740 - 0.17411041666666666666e-2 * t34784 + 0.11607361111111111111e-2 * t34787 + 0.34822083333333333332e-2 * t34790 + 0.23214722222222222222e-2 * t34807 - 0.52083333333333333333e-2 * t35026 * t2740 - 0.23214722222222222222e-2 * t34811 - t32502 + 0.11607361111111111111e-2 * t34817 + 0.40208333333333333334e-2 * t33778 * t9855 + 0.34722222222222222222e-2 * t33966 - 0.38691203703703703703e-3 * t34827;
    (t35018, t35025, t35026, t35035)
}
