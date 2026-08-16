//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2087/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2087(t90983: f64, t26432: f64, t6897: f64, t794: f64, t22642: f64, t22690: f64, t26395: f64, t22863: f64, t7737: f64, t26448: f64, t90497: f64, t215: f64, t6916: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90984 = 0.82246703342411321824e-2_f64 * t90983;
    let t90987 = t6897 * t794 * t26432;
    let t90988 = 0.82246703342411321824e-2_f64 * t90987;
    let t90993 = t22642 * t22690 * t26395;
    let t91000 = t22863 * t7737;
    let t91002 = t90497 * t26448;
    let t91004 = t6916 * t215;
    (t90984, t90988, t90993, t91000, t91002, t91004)
}
