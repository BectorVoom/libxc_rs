//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1248/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1248(t3940: f64, t94429: f64, t27940: f64, t9837: f64, t9842: f64, t26028: f64, t9832: f64, t9828: f64, t25983: f64, t9914: f64, t2482: f64, t596: f64, t7269: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94430 = t94429 * t3940;
    let t94432 = t27940 * t9837;
    let t94434 = t27940 * t9842;
    let t94436 = t26028 * t9832;
    let t94438 = t26028 * t9828;
    let t94440 = t25983 * t9914;
    let t94443 = t2482 * t7269 * t596;
    (t94430, t94432, t94434, t94436, t94438, t94440, t94443)
}
