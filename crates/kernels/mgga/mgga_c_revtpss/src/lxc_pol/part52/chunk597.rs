//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 597/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk597<F: Float>(t239: F, t7036: F, t820: F, t839: F, t1946: F, t846: F, t233: F, t64: F, t857: F, t7024: F, t7026: F, t7032: F, t7035: F) -> (F, F, F, F, F, F, F, F) {
    let t7038 = t820 * t7036 * t239;
    let t7039 = t7038 * t839;
    let t7041 = t1946 * t846;
    let t7042 = 0.20007875121765877254e-2 * t7041;
    let t7043 = t233 * t64;
    let t7045 = t820 * t7043 * t239;
    let t7046 = t7045 * t857;
    let t7048 = -t7024 - t7026 / 48.0 - t7032 + t7035 - 0.42874018118069736972e-3 * t7039 - t7042 - 0.17149607247227894789e-2 * t7046;
    (t7038, t7039, t7041, t7042, t7043, t7045, t7046, t7048)
}
