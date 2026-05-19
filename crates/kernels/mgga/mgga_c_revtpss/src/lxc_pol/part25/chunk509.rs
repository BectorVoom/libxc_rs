//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 509/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk509<F: Float>(t251: F, t2783: F, t786: F, t231: F, t268: F, t675: F, t836: F, t72: F, t860: F, t686: F, t874: F, t2470: F, t875: F) -> (F, F, F, F, F, F, F) {
    let t2797 = t2783 * t251;
    let t2798 = t786 * t2797;
    let t2801 = t268 * t675 * t836 * t231;
    let t2802 = t2798 * t2801;
    let t2804 = t860 * t72;
    let t2806 = t874 * t2804 * t686;
    let t2810 = F::cast_from(0.13009920719177044025e-1_f64) * t874 * t875 * t2470;
    (t2797, t2798, t2801, t2802, t2804, t2806, t2810)
}
