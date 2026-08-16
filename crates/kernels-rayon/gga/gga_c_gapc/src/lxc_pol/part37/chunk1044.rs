//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1044/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1044(t11259: f64, t11265: f64, t11268: f64, t11274: f64, t11276: f64, t1049: f64, t10526: f64, t10529: f64, t2967: f64, t3179: f64, t3480: f64, t1112: f64, t8598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12031 = 0.86898242813537603825e-4_f64 * t11259;
    let t12033 = 0.22776267492663374278e-4_f64 * t11265;
    let t12034 = 0.2530696388073708253e-5_f64 * t11268;
    let t12035 = 0.73811977985483157379e-6_f64 * t11274;
    let t12036 = 0.12147342662753799615e-3_f64 * t11276;
    let t12042 = t10526 * t1049;
    let t12043 = t10529 * t2967;
    let t12044 = 2.0_f64 * t12043;
    let t12045 = t3480 * t3179;
    let t12046 = t8598 * t1112;
    (t12031, t12033, t12034, t12035, t12036, t12042, t12043, t12044, t12045, t12046)
}
