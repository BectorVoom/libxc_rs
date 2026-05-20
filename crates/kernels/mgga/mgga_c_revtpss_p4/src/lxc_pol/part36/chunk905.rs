//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 905/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk905<F: Float>(t6071: F, t72: F, t686: F, t2465: F, t213: F, t6041: F, t6048: F, t10995: F, t6072: F, t779: F, t689: F, t1580: F, t4321: F) -> (F, F, F, F, F, F, F) {
    let t18796 = t6071 * t72;
    let t18797 = t18796 * t686;
    let t18798 = t2465 * t18797;
    let t18800 = t213 * t6041;
    let t18804 = t6048 * t72;
    let t18805 = t18804 * t686;
    let t18806 = t10995 * t18805;
    let t18811 = t779 * t6072;
    let t18812 = t689 * t18811;
    let t18814 = t4321 * t1580;
    (t18797, t18798, t18800, t18805, t18806, t18812, t18814)
}
