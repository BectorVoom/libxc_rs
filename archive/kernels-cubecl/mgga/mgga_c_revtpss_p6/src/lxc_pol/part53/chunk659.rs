//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 659/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk659<F: Float>(t114: F, t651: F, t6993: F, t112: F, t624: F, t655: F, t68: F, t665: F) -> (F, F, F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t6995 = F::cast_from(2.0_f64) * t651 * t6993;
    let t6996 = t624 * t112;
    let t6997 = t6996 / F::cast_from(3.0_f64);
    let t6998 = t68 * t655;
    let t6999 = t6998 * t665;
    let t7002 = piecewise3::<F>(t115, F::cast_from(0.0_f64), -t6997 - t6999 / F::cast_from(8.0_f64));
    (t6995, t6997, t6998, t7002)
}
