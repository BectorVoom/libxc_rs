//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1103/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1103(t1060: f64, t32943: f64, t30816: f64, t7577: f64, t30820: f64, t7582: f64, t1610: f64, t8387: f64, t1618: f64, t1622: f64, t1935: f64, t30813: f64, t30829: f64, t30837: f64, t30840: f64, t378: f64, t6742: f64, t7574: f64, t8384: f64) -> (f64, f64, f64, f64, f64) {
    let t32944 = t32943 * t1060;
    let t32948 = t7577 * t30816;
    let t32951 = t30820 * t7582;
    let t32954 = t1610 * t8387;
    let t32961 = t30813 + 0.40372756094140390856e-3_f64 * t7574 * t8384 - 0.40372756094140390856e-3_f64 * t1935 * t32948 + 0.40372756094140390856e-3_f64 * t6742 * t32951 + t32954 * t378 / 1536.0_f64 + t30829 * t1618 / 1536.0_f64 + t30837 + t30840 * t1622 / 2304.0_f64;
    (t32944, t32948, t32951, t32954, t32961)
}
