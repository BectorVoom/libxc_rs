//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1207/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1207(t18547: f64, t14363: f64, t162: f64, t18298: f64, t187: f64, t10563: f64, t14324: f64, t14343: f64, t14345: f64, t14372: f64, t18535: f64, t18536: f64, t18537: f64, t18538: f64, t18541: f64, t18543: f64, t18546: f64, t9394: f64) -> (f64, f64, f64, f64) {
    let t18548 = 8.0_f64 * t18547;
    let t18549 = 0.21687162600603479684e-1_f64 * t14363;
    let t18550 = t18298 * t162;
    let t18552 = 0.19751673498613801407e-1_f64 * t18550 * t187;
    let t18553 = -t14324 + t18535 - t18536 - t18537 + t18538 + t14343 + t14345 + t18541 + t18543 + t18546 + t18548 + t9394 + t18549 + t18552 + t14372 + t10563;
    (t18548, t18549, t18552, t18553)
}
