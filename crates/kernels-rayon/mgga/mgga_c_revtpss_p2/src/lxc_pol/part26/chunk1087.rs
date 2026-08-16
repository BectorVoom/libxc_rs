//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1087/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1087(t25310: f64, t7407: f64, t25305: f64, t26519: f64, t26506: f64, t7058: f64, t2471: f64, t7388: f64, t25375: f64, t26485: f64, t72: f64, t7423: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26529 = t25310 * t7407;
    let t26534 = 0.22849835011101738147e-2_f64 * t25305 * t26519;
    let t26536 = 0.96373646535613327357e-2_f64 * t7058 * t26506;
    let t26538 = 0.13009920719177044025e-1_f64 * t7388 * t2471;
    let t26541 = t25375 * t26485;
    let t26543 = t7423 * t72;
    (t26529, t26534, t26536, t26538, t26541, t26543)
}
