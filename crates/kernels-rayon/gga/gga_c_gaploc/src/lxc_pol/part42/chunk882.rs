//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 882/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk882(t2684: f64, t45320: f64, t7585: f64, t45369: f64, t7427: f64, t7573: f64, t10930: f64, t10931: f64, t326: f64, t45423: f64, t825: f64, t11832: f64, t1445: f64, t2530: f64, t5748: f64) -> (f64, f64, f64, f64, f64) {
    let t45766 = 0.87421871174939309262e2_f64 * t2684 * t7585 * t45320;
    let t45772 = 0.37959496694381542179e3_f64 * t7427 * t7573 * t45369;
    let t45775 = 0.38649669361552115674e3_f64 * t10930 * t10931 * t45369;
    let t45778 = 0.92023022289409799224e1_f64 * t825 * t326 * t45423;
    let t45785 = 0.27606906686822939767e2_f64 * t5748 * t1445 * t11832 * t2530;
    (t45766, t45772, t45775, t45778, t45785)
}
