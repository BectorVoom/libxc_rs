//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1272/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1272<F: Float>(t11916: F, t25509: F, t25569: F, t3111: F, t11722: F, t7132: F, t11727: F, t12002: F, t1971: F, t351: F, t1052: F, t3089: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t93573 = t25509 * t11916;
    let t93579 = t25569 * t3111;
    let t93583 = t7132 * t11722;
    let t93585 = t7132 * t11727;
    let t93592 = t351 * t1971 * t12002;
    let t93595 = sigma0 * t1052;
    let t93596 = t93595 * t3089;
    (t93573, t93579, t93583, t93585, t93592, t93596)
}
