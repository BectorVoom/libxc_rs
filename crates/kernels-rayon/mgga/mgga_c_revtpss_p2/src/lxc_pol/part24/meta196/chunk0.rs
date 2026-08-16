//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 927/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk927(t2482: f64, t27: f64, t4000: f64, t555: f64, t5744: f64, t786: f64, t4083: f64, t9303: f64, t123: f64, t212: f64, t2434: f64) -> (f64, f64, f64, f64, f64) {
    let t10001 = t2482 * t4000 * t27;
    let t10022 = t5744 * t555;
    let t10023 = t786 * t10022;
    let t10035 = 0.26019841438354088051e-2_f64 * t9303 * t4083;
    let t10069 = t123 * t2434 * t212;
    (t10001, t10022, t10023, t10035, t10069)
}
