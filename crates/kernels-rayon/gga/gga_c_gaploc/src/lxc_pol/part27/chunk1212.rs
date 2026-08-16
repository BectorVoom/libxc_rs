//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1212/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1212(t10731: f64, t7129: f64, t2508: f64, t32356: f64, t688: f64, t779: f64, t10682: f64, t2060: f64, t1897: f64, t27348: f64, t954: f64, t23433: f64, t2936: f64) -> (f64, f64, f64, f64, f64) {
    let t32466 = 0.18457262952341338281e0_f64 * t7129 * t10731;
    let t32471 = 0.15381052460284448567e-1_f64 * t2508 * t779 * t32356 * t688;
    let t32474 = 0.76905262301422242837e-2_f64 * t2508 * t2060 * t10682;
    let t32477 = 0.76905262301422242837e-2_f64 * t1897 * t954 * t27348;
    let t32480 = 0.23071578690426672851e-1_f64 * t1897 * t2936 * t23433;
    (t32466, t32471, t32474, t32477, t32480)
}
