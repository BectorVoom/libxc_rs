//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 873/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk873<F: Float>(t25975: F, t26002: F, t26010: F, t26012: F, t26021: F, t212: F, t7506: F, t1358: F, t689: F, t2097: F, t785: F, t2439: F) -> (F, F, F, F, F, F, F) {
    let t26312 = F::new(0.22675591804667994221e-1) * t25975;
    let t26321 = F::new(35.0) / F::new(216.0) * t26002;
    let t26324 = F::new(0.10164000561857065645e-4) * t26010;
    let t26325 = F::new(0.30488190661738479625e-3) * t26012;
    let t26328 = F::new(0.18071592998981862717e-4) * t26021;
    let t26354 = t212 * t7506;
    let t26355 = t26354 * t1358;
    let t26356 = t689 * t26355;
    let t26358 = t785 * t2097;
    let t26359 = t26358 * t1358;
    let t26361 = F::new(0.65049603595885220126e-3) * t2439 * t26359;
    (t26312, t26321, t26324, t26325, t26328, t26356, t26361)
}
