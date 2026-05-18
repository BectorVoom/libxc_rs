//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1206/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1206<F: Float>(t3335: F, t11198: F, t340: F, t11119: F, t384: F, t11238: F, t196: F, t10296: F, t602: F, t2240: F, t2246: F, t10308: F, t599: F) -> (F, F, F, F, F, F, F) {
    let t41936 = t3335 * t3335;
    let t41937 = F::new(1.0) / t41936;
    let t42058 = F::new(1.0) / t11198 / t340;
    let t42066 = F::new(1.0) / t11119 / t384;
    let t42859 = F::new(1.0) / t11238 / t196;
    let t45955 = t10296 * t602;
    let t45958 = t2240 * t2246;
    let t45963 = t599 * t10308;
    (t41937, t42058, t42066, t42859, t45955, t45958, t45963)
}
