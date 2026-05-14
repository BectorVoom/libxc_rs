//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 760/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk760<F: Float>(t1558: F, t2723: F, t836: F, t136: F, t243: F, t220: F, t125: F, t4343: F, t221: F, t4433: F, t1501: F, t670: F) -> (F, F, F, F, F, F) {
    let t14586 = t1558 * t2723;
    let t14587 = t14586 * t836;
    let t14685 = t243 * t136;
    let t14686 = t14685 * t220;
    let t14691 = t125 * t4343;
    let t14756 = t221 * t4433;
    let t18227 = t1501 * t670;
    (t14587, t14685, t14686, t14691, t14756, t18227)
}
