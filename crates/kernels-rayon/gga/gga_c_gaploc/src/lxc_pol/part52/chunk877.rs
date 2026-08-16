//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 877/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk877(t2615: f64, t326: f64, t45316: f64, t43832: f64, t1890: f64, t1966: f64, t44707: f64, t590: f64, t10024: f64, t11823: f64, t43881: f64, t5241: f64, t5640: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45658 = 0.92023022289409799224e1_f64 * t2615 * t326 * t45316;
    let t45663 = 0.23005755572352449806e1_f64 * t43832;
    let t45667 = 0.97135412416599232513e1_f64 * t1966 * t1890 * t44707 * t590;
    let t45678 = t11823 * t10024;
    let t45680 = 0.15337170381568299871e1_f64 * t43881;
    let t45684 = 0.13803453343411469884e2_f64 * t5640 * t5241 * t44707 * t590;
    (t45658, t45663, t45667, t45678, t45680, t45684)
}
