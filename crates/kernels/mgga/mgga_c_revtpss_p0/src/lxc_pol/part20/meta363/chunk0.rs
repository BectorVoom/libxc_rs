//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1319/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1319<F: Float>(t39756: F, t39760: F, t39764: F, t39767: F, t39770: F, t39773: F, t39775: F, t39778: F, t39780: F, t39783: F, t39786: F, t268: F, t681: F, t702: F, t793: F) -> (F, F) {
    let t39787 = t39756 + t39760 - t39764 + t39767 + t39770 + t39773 - t39775 + t39778 + t39780 - t39783 - t39786;
    let t39791 = F::cast_from(0.22161481481481481481e0_f64) * t268 * t793 * t681 * t702;
    (t39787, t39791)
}
