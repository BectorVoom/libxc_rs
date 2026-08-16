//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1832/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1832(t2482: f64, t596: f64, t7043: f64, t2677: f64, t240: f64, t25260: f64, t25228: f64, t9775: f64, t233: f64, t41077: f64, t7056: f64, t9646: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93072 = t2482 * t7043 * t596;
    let t93073 = t93072 * t2677;
    let t93082 = t25260 * t240;
    let t93088 = t9775 * t25228;
    let t93118 = t41077 * t233;
    let t93134 = t9646 * t7056;
    (t93072, t93073, t93082, t93088, t93118, t93134)
}
