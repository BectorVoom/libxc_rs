//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1137/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1137(t241: f64, t25981: f64, t820: f64, t25997: f64, t5665: f64, t1873: f64, t26004: f64, t1904: f64, t7242: f64, t689: f64, t786: f64, t7911: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27940 = t820 * t25981 * t241;
    let t27953 = t25997 * t5665;
    let t27955 = t26004 * t1873;
    let t27965 = t7242 * t1904;
    let t27966 = t689 * t27965;
    let t27968 = t786 * t7911;
    (t27940, t27953, t27955, t27965, t27966, t27968)
}
