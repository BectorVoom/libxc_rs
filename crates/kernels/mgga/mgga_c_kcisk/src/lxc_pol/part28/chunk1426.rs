//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1426/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1426<F: Float>(t113003: F, t117621: F, t117951: F, t118064: F, t118070: F, t118275: F, t121440: F, t122539: F, t2807: F, t34400: F, t34406: F, t34424: F, t34429: F, t34435: F, t35462: F, t35463: F, t9720: F, t9739: F, t9740: F, t9748: F) -> (F,) {
    let t122799 = -0.61728395061728395062e-2 * t118064 - 0.20833333333333333334e-1 * t9740 * t122539 - 0.40208333333333333334e-2 * t118275 * t34400 - 0.120625e-1 * t118275 * t34406 - t113003 - 0.89351851851851851853e-3 * t118070 - 0.23280625e-2 * t117951 * t9739 * t34406 - 0.10416666666666666667e-1 * t34435 * t34400 - 0.20833333333333333334e-1 * t34435 * t34424 - 0.10416666666666666667e-1 * t34435 * t34429 - 0.40208333333333333335e-2 * t117621 * t34429 - 0.52083333333333333333e-2 * t9720 * t35462 * t2807 + 0.69644166666666666664e-2 * t121440 + 0.52083333333333333333e-2 * t35463 * t9748;
    (t122799,)
}
