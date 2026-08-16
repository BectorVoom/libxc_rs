//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1207/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1207(t239: f64, t820: f64, t94491: f64, t2482: f64, t596: f64, t7262: f64, t25981: f64, t27: f64, t550: f64, t7021: f64, t1412: f64, t1941: f64) -> (f64, f64, f64, f64, f64) {
    let t94493 = t820 * t94491 * t239;
    let t94497 = t2482 * t7262 * t596;
    let t94508 = t2482 * t25981 * t27;
    let t94513 = t7021 * t550;
    let t94516 = t1941 * t1412;
    (t94493, t94497, t94508, t94513, t94516)
}
