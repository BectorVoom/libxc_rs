//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1768/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1768(t25234: f64, t2677: f64, t1941: f64, t243: f64, t2732: f64, t2712: f64, t64: f64) -> (f64, f64, f64, f64) {
    let t25235 = t25234 * t2677;
    let t25236 = 0.2032800112371413129e-3_f64 * t25235;
    let t25237 = t1941 * t243;
    let t25238 = t25237 * t2732;
    let t25240 = t64 * t2712;
    (t25235, t25236, t25238, t25240)
}
