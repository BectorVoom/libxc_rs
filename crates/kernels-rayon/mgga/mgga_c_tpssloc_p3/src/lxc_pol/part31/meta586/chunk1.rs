//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1828/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1828(t1831: f64, t80869: f64, t22783: f64, t5314: f64, t26297: f64, t80853: f64, t80855: f64, t26301: f64, t80866: f64, t131: f64, t6931: f64, t9537: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91135 = t80869 * t1831;
    let t91137 = t22783 * t5314;
    let t91140 = t80853 * t80855 * t26297;
    let t91143 = t80853 * t80855 * t26301;
    let t91149 = t80866 * t1831;
    let t91152 = t6931 * t131 * t9537;
    (t91135, t91137, t91140, t91143, t91149, t91152)
}
