//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 888/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk888<F: Float>(t31799: F, t31801: F, t31748: F, t31751: F, t31759: F, t31764: F, t31770: F, t31775: F, t31783: F, t31786: F, t31787: F, t31791: F, t31794: F, t31795: F, t7083: F, t8472: F) -> (F, F) {
    let t31803 = 0.14279934416275588154e-1 * t31799 * t31801;
    let t31804 = -t31748 + t31751 - 0.28234466758480466999e-3 * t31759 - t31764 - 0.112937867033921868e-2 * t31770 - 0.28234466758480466999e-3 * t31775 + t31783 - t31786 - 0.17347256376410398924e1 * t31787 * t7083 + 0.17347256376410398924e1 * t8472 * t31791 + 0.8673628188205199462e0 * t31794 * t31795 - t31803;
    (t31803, t31804)
}
