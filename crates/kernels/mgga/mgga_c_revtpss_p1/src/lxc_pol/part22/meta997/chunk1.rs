//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3388/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3388<F: Float>(t15474: F, t1610: F, t2874: F, t11299: F, t2918: F, t6145: F, t11528: F, t19327: F, t19128: F, t934: F, t6142: F, t19330: F, t2875: F) -> (F, F, F, F, F, F) {
    let t63633 = F::cast_from(4.0_f64) * t2874 * t1610 * t15474;
    let t63636 = F::cast_from(0.96491876992155210402e2_f64) * t11299 * t6145 * t2918;
    let t63638 = F::cast_from(4.0_f64) * t11528 * t19327;
    let t63641 = F::cast_from(4.0_f64) * t2874 * t19128 * t934;
    let t63644 = F::cast_from(2.0_f64) * t2874 * t6142 * t2918;
    let t63647 = F::cast_from(0.96491876992155210402e2_f64) * t11299 * t19330 * t2875;
    (t63633, t63636, t63638, t63641, t63644, t63647)
}
