//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2060/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2060<F: Float>(t94570: F, t1445: F, t2439: F, t25916: F, t1358: F, t212: F, t26034: F, t689: F, t25877: F, t94390: F, t94385: F, t9675: F) -> (F, F, F, F, F) {
    let t94571 = F::cast_from(0.14450132032386466905e-2_f64) * t94570;
    let t94580 = t2439 * t25916 * t1445;
    let t94584 = t689 * t212 * t26034 * t1358;
    let t94589 = t94390 * t25877;
    let t94590 = t94385 * t9675;
    (t94571, t94580, t94584, t94589, t94590)
}
