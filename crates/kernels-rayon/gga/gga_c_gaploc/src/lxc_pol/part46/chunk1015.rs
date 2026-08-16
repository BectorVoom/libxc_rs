//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1015/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1015(t10677: f64, t2464: f64, t2465: f64, t825: f64, t10782: f64, t2684: f64, t13072: f64, t32757: f64, t25359: f64, t2615: f64, t9438: f64, t1445: f64, t3209: f64, t833: f64, t8469: f64) -> (f64, f64, f64, f64, f64) {
    let t44124 = t825 * t2464 * t2465 * t10677;
    let t44128 = t2684 * t2464 * t2465 * t10782;
    let t44130 = t32757 * t13072;
    let t44131 = 0.89376224879626066675e-1_f64 * t44130;
    let t44133 = t2615 * t9438 * t25359;
    let t44134 = 0.15976219147466979032e-1_f64 * t44133;
    let t44138 = 0.43710935587469654631e2_f64 * t833 * t1445 * t8469 * t3209;
    (t44124, t44128, t44131, t44134, t44138)
}
