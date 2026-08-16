//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1107/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1107(t14159: f64, t2457: f64, t3964: f64, t2435: f64, t5760: f64, t545: f64, t5710: f64, t869: f64, t689: f64, t225: f64, t9990: f64, t213: f64) -> (f64, f64, f64, f64) {
    let t14161 = t3964 * t14159 * t2457;
    let t14166 = t2435 * t5760;
    let t14188 = t545 * t5710;
    let t14189 = t869 * t14188;
    let t14191 = 0.10975748638225852664e-1_f64 * t689 * t14189;
    let t14192 = t225 * t9990;
    let t14193 = t213 * t14192;
    (t14161, t14166, t14191, t14193)
}
