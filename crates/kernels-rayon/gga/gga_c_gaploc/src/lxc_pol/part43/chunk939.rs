//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 939/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk939(t44117: f64, t13072: f64, t32757: f64, t25359: f64, t2615: f64, t9438: f64, t1445: f64, t3209: f64, t833: f64, t8469: f64, t25405: f64, t5748: f64) -> (f64, f64, f64, f64, f64) {
    let t44118 = 0.15976219147466979032e-1_f64 * t44117;
    let t44130 = t32757 * t13072;
    let t44133 = t2615 * t9438 * t25359;
    let t44134 = 0.15976219147466979032e-1_f64 * t44133;
    let t44138 = 0.43710935587469654631e2_f64 * t833 * t1445 * t8469 * t3209;
    let t44142 = 0.27606906686822939767e2_f64 * t5748 * t1445 * t25405 * t3209;
    (t44118, t44130, t44134, t44138, t44142)
}
