//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 711/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk711<F: Float>(t114: F, t651: F, t6993: F, t112: F, t624: F, t655: F, t68: F, t665: F) -> (F, F, F) {
    let t115 = F::new(1.0) < t114;
    let t6995 = F::new(2.0) * t651 * t6993;
    let t6996 = t624 * t112;
    let t6997 = t6996 / F::new(3.0);
    let t6998 = t68 * t655;
    let t6999 = t6998 * t665;
    let t7002 = piecewise3::<f64>(t115, F::new(0.0), -t6997 - t6999 / F::new(8.0));
    (t6995, t6998, t7002)
}
