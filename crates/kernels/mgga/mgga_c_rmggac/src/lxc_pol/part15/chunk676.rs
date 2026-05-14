//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 676/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk676<F: Float>(t20925: F, t253: F, t34747: F, t34750: F, t7321: F, t4765: F, t49: F, t7322: F, t388: F, t140: F, t673: F, t465: F, t7472: F, t7716: F, t20: F, t2018: F, t2021: F, t4747: F) -> (F, F, F, F, F, F, F, F) {
    let t34752 = t253 * t34747 * t7321 * t20925 * t34750;
    let t34753 = 0.10260057759007034251e-5 * t34752;
    let t34755 = t4765 * t7322 * t49;
    let t34757 = t34755 * t388 * t34750;
    let t34759 = t673 * t140;
    let t34760 = t465 * t34759;
    let t34761 = t7472 * t34760;
    let t34764 = t7716 * t34760;
    let t34772 = t4747 * t20 * t2018 * t2021;
    (t34753, t34755, t34757, t34759, t34760, t34761, t34764, t34772)
}
