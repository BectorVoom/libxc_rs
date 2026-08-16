//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1863/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1863<F: Float>(t7289: F, t96282: F, t26277: F, t94776: F, t25950: F, t26292: F, t25904: F, t96245: F, t94471: F, t94473: F, t94476: F, t94483: F) -> (F, F, F, F, F, F, F, F) {
    let t96284 = F::cast_from(0.39982213492741449076e-1_f64) * t7289 * t96282;
    let t96287 = t94776 * t26277;
    let t96289 = t25950 * t26292;
    let t96298 = t25904 * t96245;
    let t96321 = F::cast_from(455.0_f64) / F::cast_from(648.0_f64) * t94471;
    let t96322 = F::cast_from(0.51384669507166276316e-2_f64) * t94473;
    let t96323 = F::cast_from(0.3252886739816735289e-3_f64) * t94476;
    let t96326 = F::cast_from(0.18295201011342718161e-3_f64) * t94483;
    (t96284, t96287, t96289, t96298, t96321, t96322, t96323, t96326)
}
