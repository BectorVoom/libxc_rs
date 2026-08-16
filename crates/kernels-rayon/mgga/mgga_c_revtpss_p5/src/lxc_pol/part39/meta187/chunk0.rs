//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 796/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk796(t1398: f64, t555: f64, t4086: f64, t543: f64, t2782: f64, t1419: f64, t545: f64, t869: f64, t689: f64, t136: f64, t2457: f64, t3964: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4087 = t555 * t1398;
    let t4089 = t4086 * t4087 * t543;
    let t4090 = t2782 * t4089;
    let t4092 = t545 * t1419;
    let t4093 = t869 * t4092;
    let t4094 = t689 * t4093;
    let t4096 = t555 * t136;
    let t4099 = 0.11565819519348392139e-2_f64 * t3964 * t4096 * t2457;
    (t4089, t4090, t4092, t4093, t4094, t4096, t4099)
}
