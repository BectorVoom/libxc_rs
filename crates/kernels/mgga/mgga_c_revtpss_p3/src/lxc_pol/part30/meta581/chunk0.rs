//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2035/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2035<F: Float>(t94564: F, t9795: F, t2018: F, t40688: F, t46808: F, t7256: F, t9784: F, t1445: F, t2439: F, t25916: F, t1358: F, t212: F, t26034: F, t689: F) -> (F, F, F, F, F) {
    let t94565 = t94564 * t9795;
    let t94568 = t40688 * t2018 * t46808;
    let t94569 = F::cast_from(0.22589491248727328397e-6_f64) * t94568;
    let t94570 = t9784 * t7256;
    let t94571 = F::cast_from(0.14450132032386466905e-2_f64) * t94570;
    let t94580 = t2439 * t25916 * t1445;
    let t94584 = t689 * t212 * t26034 * t1358;
    (t94565, t94569, t94571, t94580, t94584)
}
