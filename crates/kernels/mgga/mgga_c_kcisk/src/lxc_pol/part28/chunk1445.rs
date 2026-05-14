//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1445/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1445<F: Float>(t35463: F, t9736: F, t113123: F, t118443: F, t122065: F, t122068: F, t122071: F, t122074: F, t122102: F, t122759: F, t122764: F, t123220: F, t2807: F, t33258: F, t34412: F, t34469: F, t34477: F, t34563: F, t34594: F, t35438: F, t35476: F, t35511: F, t9720: F, t9748: F, t9995: F) -> (F,) {
    let t123296 = t35463 * t9736;
    let t123312 = 0.23214722222222222221e-2 * t122065 + 0.92858888888888888886e-2 * t122068 - 0.61905925925925925924e-2 * t122071 + 0.23214722222222222222e-2 * t122074 + 0.40208333333333333334e-2 * t34594 * t34469 - 0.27777777777777777778e-1 * t35511 * t9748 + 0.10416666666666666667e-1 * t34477 * t9995 - 0.17361111111111111111e-2 * t123296 + 0.12345679012345679012e-1 * t34412 * t34563 - 0.26805555555555555556e-2 * t113123 * t122764 - 0.77602083333333333335e-3 * t118443 * t122759 + 0.17870370370370370371e-2 * t113123 * t123220 - 0.61905925925925925924e-2 * t122102 - 0.50925925925925925926e-1 * t9720 * t35438 * t2807 + 0.20104166666666666667e-2 * t33258 * t35476;
    (t123312,)
}
