//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1220/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1220(t10737: f64, t5288: f64, t1949: f64, t3437: f64, t1944: f64, t24745: f64, t5539: f64, t9647: f64, t123: f64, t24884: f64, t2563: f64, t10697: f64, t7173: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32681 = 0.30762104920568897134e-1_f64 * t5288 * t10737;
    let t32682 = t1949 * t3437;
    let t32683 = 0.85450291446024714264e-3_f64 * t32682;
    let t32684 = t1944 * t3437;
    let t32685 = 0.99692006687028833308e-3_f64 * t32684;
    let t32690 = t9647 * t5539 * t24745;
    let t32691 = 0.64087718584518535698e-3_f64 * t32690;
    let t32692 = t24884 * t123;
    let t32694 = t9647 * t32692 * t2563;
    let t32695 = 0.19226315575355560709e-2_f64 * t32694;
    let t32697 = t9647 * t10697 * t7173;
    (t32681, t32683, t32685, t32691, t32695, t32697)
}
