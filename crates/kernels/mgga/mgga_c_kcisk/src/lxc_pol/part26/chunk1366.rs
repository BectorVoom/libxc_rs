//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1366/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1366<F: Float>(t119795: F, t119797: F, t119799: F, t119801: F, t119803: F, t119805: F, t119807: F, t119809: F, t119811: F, t119813: F, t119815: F, t119817: F, t119819: F, t119821: F, t119823: F, t119825: F, t119828: F, t119830: F) -> (F,) {
    let t119966 = -0.53958333333333333334e-1 * t119795 + 0.20234375e-1 * t119797 - 0.5625e0 * t119799 + 0.125e0 * t119801 - 0.16666666666666666667e0 * t119803 - 0.125e0 * t119805 - 0.625e-1 * t119807 + 0.4046875e-1 * t119809 + 0.41666666666666666667e-1 * t119811 - 0.89930555555555555557e-2 * t119813 + 0.14388888888888888889e0 * t119815 + 0.28777777777777777778e0 * t119817 + 0.9375e-1 * t119819 + 0.55555555555555555555e-1 * t119821 - 0.20833333333333333333e-1 * t119823 + 0.33333333333333333333e0 * t119825 + 0.625e-1 * t119828 + 0.59953703703703703705e-2 * t119830;
    (t119966,)
}
