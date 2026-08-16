//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2143/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2143(t25260: f64, t4368: f64, t820: f64, t844: f64, t14914: f64, t25270: f64, t14919: f64, t14904: f64, t27261: f64, t14900: f64, t4462: f64, t92951: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98937 = t820 * t25260 * t844 * t4368;
    let t98940 = t25270 * t14914;
    let t98943 = t25270 * t14919;
    let t98945 = t27261 * t14904;
    let t98947 = t27261 * t14900;
    let t98949 = t92951 * t4462;
    (t98937, t98940, t98943, t98945, t98947, t98949)
}
