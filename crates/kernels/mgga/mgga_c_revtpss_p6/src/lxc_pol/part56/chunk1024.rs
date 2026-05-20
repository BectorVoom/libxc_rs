//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1024/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1024<F: Float>(t1568: F, t7063: F, t33: F, t41154: F, t116: F, t29421: F, t1203: F, t471: F, t11239: F, t1811: F, t1828: F, t1774: F) -> (F, F, F, F, F, F, F) {
    let t98848 = t7063 * t1568;
    let t100981 = t41154 * t33;
    let t104115 = t29421 * t116;
    let t104504 = t471 * t1203;
    let t104527 = t1811 * t11239;
    let t105236 = t1828 * t1203;
    let t105270 = t1774 * t1203;
    (t98848, t100981, t104115, t104504, t104527, t105236, t105270)
}
