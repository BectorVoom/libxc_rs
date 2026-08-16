//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2098/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2098(t26072: f64, t27888: f64, t27873: f64, t94886: f64, t27845: f64, t689: f64, t25904: f64, t25899: f64, t94649: f64, t97685: f64, t25898: f64, t7925: f64, t94849: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97943 = 0.14456046980341999104e-1_f64 * t26072 * t27888;
    let t97945 = 0.51405703062096148812e-1_f64 * t94886 * t27873;
    let t97947 = t27845 * t689;
    let t97949 = 0.14456046980341999104e-1_f64 * t25904 * t97947;
    let t97951 = 0.25702851531048074406e-1_f64 * t25899 * t97947;
    let t97953 = 0.51405703062096148812e-1_f64 * t94649 * t97685;
    let t97956 = t94849 * t25898 * t7925;
    (t97943, t97945, t97949, t97951, t97953, t97956)
}
