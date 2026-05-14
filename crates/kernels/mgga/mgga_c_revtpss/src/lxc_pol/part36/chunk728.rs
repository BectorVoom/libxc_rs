//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 728/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk728<F: Float>(t651: F, t7742: F, t1544: F, t30: F, t1963: F, t1549: F, t7025: F, t1561: F, t7038: F, t1565: F, t7045: F, t7024: F, t7032: F, t7035: F, t7042: F) -> (F, F, F, F) {
    let t7744 = 2.0 * t651 * t7742;
    let t7749 = t30 * t1544;
    let t7750 = t1963 * t7749;
    let t7753 = t7025 * t1549;
    let t7755 = t7038 * t1561;
    let t7757 = t7045 * t1565;
    let t7759 = -t7024 - t7753 / 48.0 - t7032 + t7035 - 0.42874018118069736972e-3 * t7755 - t7042 - 0.17149607247227894789e-2 * t7757;
    (t7744, t7749, t7750, t7759)
}
