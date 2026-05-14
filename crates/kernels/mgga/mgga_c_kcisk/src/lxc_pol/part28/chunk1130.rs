//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1130/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1130<F: Float>(t32664: F, t9382: F, t9368: F, t15492: F, t9364: F, t32646: F, t3373: F, t32643: F, t9365: F, t32629: F, t32634: F, t32638: F, t32640: F, t32644: F, t32648: F, t32650: F, t32653: F, t32655: F, t32659: F, t32662: F) -> (F, F, F) {
    let t32665 = t32664 * t9382;
    let t32667 = t32664 * t9368;
    let t32669 = t15492 * t9364;
    let t32670 = t32669 * t9368;
    let t32672 = t3373 * t32646;
    let t32673 = t32672 * t9368;
    let t32675 = t9365 * t32643;
    let t32677 = -0.20833333333333333334e-1 * t32629 - 0.20833333333333333334e-1 * t32634 - 0.18763888888888888889e-1 * t32638 - 0.120625e-1 * t32640 + 0.10416666666666666667e-1 * t32644 + 0.20833333333333333334e-1 * t32648 + 0.20833333333333333334e-1 * t32650 - 0.48611111111111111112e-1 * t32653 - 0.48611111111111111112e-1 * t32655 + 0.10416666666666666667e-1 * t32659 - 0.23280625000000000001e-2 * t32662 + 0.20833333333333333334e-1 * t32665 + 0.20833333333333333334e-1 * t32667 + 0.8041666666666666667e-2 * t32670 + 0.8041666666666666667e-2 * t32673 + 0.40208333333333333335e-2 * t32675;
    (t32669, t32672, t32677)
}
