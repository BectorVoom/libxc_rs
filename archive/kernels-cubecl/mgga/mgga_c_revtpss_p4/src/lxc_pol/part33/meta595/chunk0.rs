//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2013/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2013<F: Float>(t2018: F, t40688: F, t46808: F, t7256: F, t9784: F, t1445: F, t2439: F, t25916: F, t25877: F, t94390: F, t94385: F, t9675: F) -> (F, F, F, F, F) {
    let t94568 = t40688 * t2018 * t46808;
    let t94569 = F::cast_from(0.22589491248727328397e-6_f64) * t94568;
    let t94570 = t9784 * t7256;
    let t94571 = F::cast_from(0.14450132032386466905e-2_f64) * t94570;
    let t94580 = t2439 * t25916 * t1445;
    let t94589 = t94390 * t25877;
    let t94590 = t94385 * t9675;
    (t94569, t94571, t94580, t94589, t94590)
}
