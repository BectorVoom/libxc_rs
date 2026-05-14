//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1205/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1205<F: Float>(t25178: F, t7235: F, t10416: F, t7003: F, t1937: F, t49693: F, t13435: F, t6993: F, t10263: F, t13216: F, t2322: F, t2331: F, t2372: F, t25078: F, t25800: F, t25805: F, t25872: F, t4254: F, t649: F, t6985: F, t7007: F, t95032: F, t95036: F, t95038: F, t95040: F, t95042: F, t95046: F, t95049: F, t95056: F) -> (F,) {
    let t95058 = 6.0 * t7235 * t25178;
    let t95066 = 6.0 * t10416 * t7003;
    let t95068 = 6.0 * t49693 * t1937;
    let t95070 = 12.0 * t13435 * t6993;
    let t95071 = -6.0 * t10263 * t6985 - 6.0 * t10416 * t7007 - 6.0 * t13216 * t6985 - 12.0 * t2322 * t25872 - 12.0 * t2331 * t25805 - 6.0 * t2372 * t25805 - 6.0 * t25078 * t4254 - 3.0 * t25800 * t649 - t95032 + t95036 - t95038 - t95040 - t95042 + t95046 - t95049 + t95056 + t95058 - t95066 - t95068 - t95070;
    (t95071,)
}
