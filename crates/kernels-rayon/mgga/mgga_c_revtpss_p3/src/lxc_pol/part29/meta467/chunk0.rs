//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1724/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1724(t2061: f64, t785: f64, t780: f64, t2439: f64, t2435: f64, t7385: f64, t2828: f64, t7071: f64, t212: f64, t7398: f64, t689: f64, t25219: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26434 = t785 * t2061;
    let t26435 = t26434 * t780;
    let t26437 = 0.65049603595885220126e-3_f64 * t2439 * t26435;
    let t26439 = 0.73171657588172351096e-2_f64 * t2435 * t7385;
    let t26440 = t2061 * t2828;
    let t26441 = t7071 * t26440;
    let t26446 = t212 * t7398;
    let t26447 = t26446 * t780;
    let t26448 = t689 * t26447;
    let t26450 = 0.22675591804667994221e-1_f64 * t25219;
    (t26434, t26435, t26437, t26439, t26440, t26441, t26446, t26447, t26448, t26450)
}
