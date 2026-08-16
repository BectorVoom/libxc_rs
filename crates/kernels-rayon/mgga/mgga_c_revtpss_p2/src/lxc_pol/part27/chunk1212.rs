//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1212/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1212(t2482: f64, t596: f64, t7043: f64, t2677: f64, t10737: f64, t7045: f64, t10741: f64, t25234: f64, t10709: f64, t25227: f64, t2661: f64, t240: f64, t25260: f64) -> (f64, f64, f64, f64, f64) {
    let t93072 = t2482 * t7043 * t596;
    let t93073 = t93072 * t2677;
    let t93075 = t7045 * t10737;
    let t93077 = t25234 * t10741;
    let t93080 = t2661 * t25227 * t10709;
    let t93082 = t25260 * t240;
    (t93073, t93075, t93077, t93080, t93082)
}
