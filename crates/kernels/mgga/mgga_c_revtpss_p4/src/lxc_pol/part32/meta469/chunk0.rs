//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1696/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1696<F: Float>(t26292: F, t7289: F, t25969: F, t25975: F, t26002: F, t26010: F, t26012: F, t26021: F, t212: F, t7506: F, t1358: F, t689: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26309 = F::cast_from(0.17135234354032049604e-1_f64) * t7289 * t26292;
    let t26310 = F::cast_from(0.54208002996571016773e-3_f64) * t25969;
    let t26312 = F::cast_from(0.22675591804667994221e-1_f64) * t25975;
    let t26321 = F::new(35.0) / F::new(216.0) * t26002;
    let t26324 = F::cast_from(0.10164000561857065645e-4_f64) * t26010;
    let t26325 = F::cast_from(0.30488190661738479625e-3_f64) * t26012;
    let t26328 = F::cast_from(0.18071592998981862717e-4_f64) * t26021;
    let t26354 = t212 * t7506;
    let t26355 = t26354 * t1358;
    let t26356 = t689 * t26355;
    (t26309, t26310, t26312, t26321, t26324, t26325, t26328, t26354, t26355, t26356)
}
