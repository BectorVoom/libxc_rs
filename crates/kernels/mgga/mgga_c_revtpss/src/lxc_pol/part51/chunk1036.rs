//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1036/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1036<F: Float>(t120013: F, t120016: F, t814: F, t853: F, t802: F, t31827: F, t844: F, t31853: F, t8486: F, t11007: F, t8477: F, t8648: F) -> (F, F, F, F, F, F, F) {
    let t120017 = t120016 * t120013;
    let t120042 = t814 * t853;
    let t120043 = t120042 * t802;
    let t120044 = t31827 * t120043;
    let t120046 = t844 * t853;
    let t120048 = t8486 * t120046 * t31853;
    let t120057 = t8477 * t8648 * t11007;
    (t120017, t120042, t120043, t120044, t120046, t120048, t120057)
}
