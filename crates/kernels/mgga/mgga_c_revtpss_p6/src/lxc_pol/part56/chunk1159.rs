//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1159/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1159<F: Float>(t32822: F, t7937: F, t28177: F, t8764: F, t34399: F, t7239: F, t8763: F, t8995: F, t28199: F, t2163: F, t28042: F, t651: F) -> (F, F, F, F, F) {
    let t129339 = t32822 * t7937;
    let t129342 = t8764 * t28177;
    let t129344 = t34399 * t7239;
    let t129353 = t8763 * t8995;
    let t129354 = t129353 * t28199;
    let t129357 = t651 * t2163 * t28042;
    (t129339, t129342, t129344, t129354, t129357)
}
