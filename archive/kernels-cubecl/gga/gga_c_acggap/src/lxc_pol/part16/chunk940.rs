//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 940/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk940<F: Float>(t525: F, t879: F, t7932: F, t7942: F, t2131: F, t2147: F, t309: F, t8436: F, t2351: F, t7924: F, t463: F, t8422: F) -> (F, F, F, F, F) {
    let t33509 = t525 * t879;
    let t33511 = t7942 * t7932 * t33509;
    let t33516 = F::cast_from(0.34694512752820797848e1_f64) * t2131 * t2147 * t8436 * t309;
    let t33518 = t7924 * t2351;
    let t33523 = F::cast_from(0.34694512752820797848e1_f64) * t2131 * t2147 * t8422 * t463;
    (t33509, t33511, t33516, t33518, t33523)
}
