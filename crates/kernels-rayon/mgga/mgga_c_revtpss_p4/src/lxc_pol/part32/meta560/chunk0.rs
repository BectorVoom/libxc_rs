//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1879/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1879(t1468: f64, t2411: f64, t30: f64, t41154: f64, t14495: f64, t689: f64, t14587: f64, t27312: f64, t1568: f64, t7063: f64, t25410: f64, t25304: f64, t27212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t98658 = t2411 * t1468;
    let t98785 = t41154 * t30;
    let t98801 = t14495 * t689;
    let t98809 = t14587 * t689;
    let t98815 = t27312 * t689;
    let t98848 = t7063 * t1568;
    let t98849 = t98848 * t25410;
    let t98867 = t25304 * t27212;
    (t98658, t98785, t98801, t98809, t98815, t98848, t98849, t98867)
}
