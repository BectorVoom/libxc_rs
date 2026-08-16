//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1183/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1183(t25299: f64, t95793: f64, t25431: f64, t95785: f64, t95789: f64, t26555: f64, t40270: f64, t25305: f64, t25410: f64, t7419: f64, t93240: f64, t26519: f64, t93160: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95794 = t25299 * t95793;
    let t95796 = t25431 * t95785;
    let t95798 = t25431 * t95789;
    let t95807 = 0.96373646535613327356e-3_f64 * t40270 * t26555;
    let t95808 = t25305 * t95793;
    let t95811 = t93240 * t25410 * t7419;
    let t95813 = t93160 * t26519;
    (t95794, t95796, t95798, t95807, t95808, t95811, t95813)
}
