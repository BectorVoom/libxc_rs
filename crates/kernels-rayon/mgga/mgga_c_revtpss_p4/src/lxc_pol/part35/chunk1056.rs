//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1056/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1056(t2061: f64, t785: f64, t780: f64, t2439: f64, t2435: f64, t7385: f64, t25219: f64, t25231: f64, t25242: f64, t25253: f64, t25275: f64, t25283: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26434 = t785 * t2061;
    let t26435 = t26434 * t780;
    let t26437 = 0.65049603595885220126e-3_f64 * t2439 * t26435;
    let t26439 = 0.73171657588172351096e-2_f64 * t2435 * t7385;
    let t26450 = 0.22675591804667994221e-1_f64 * t25219;
    let t26454 = 0.54208002996571016773e-3_f64 * t25231;
    let t26457 = 0.18071592998981862717e-4_f64 * t25242;
    let t26462 = 0.30488190661738479625e-3_f64 * t25253;
    let t26468 = 35.0_f64 / 216.0_f64 * t25275;
    let t26471 = 0.10164000561857065645e-4_f64 * t25283;
    (t26434, t26435, t26437, t26439, t26450, t26454, t26457, t26462, t26468, t26471)
}
