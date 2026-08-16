//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1967/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1967(t1353: f64, t6781: f64, t30122: f64, t1450: f64, t21969: f64, t1518: f64, t4245: f64, t1501: f64, t4292: f64, t1448: f64, t21881: f64, t93: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t109100 = t6781 * t1353;
    let t109104 = t30122 * t1353;
    let t109118 = t1450 * t21969;
    let t109150 = t4245 * t1518;
    let t109153 = t1501 * t4292;
    let t109199 = t30122 * t1448;
    let t109242 = t93 * t21881;
    (t109100, t109104, t109118, t109150, t109153, t109199, t109242)
}
