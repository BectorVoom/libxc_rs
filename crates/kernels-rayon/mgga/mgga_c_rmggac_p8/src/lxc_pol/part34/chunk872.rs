//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 872/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk872(t14368: f64, t15223: f64, t15227: f64, t70279: f64, t1550: f64, t2060: f64, t40983: f64, t69894: f64, t27: f64, t9151: f64, t16064: f64, t69609: f64) -> (f64, f64, f64, f64, f64) {
    let t75596 = t14368 * t15223;
    let t75598 = t70279 * t15227;
    let t75602 = 0.5987120850931904282e-1_f64 * t1550 * t2060 * t40983;
    let t75607 = 0.79828278012425390427e-1_f64 * t69894;
    let t75609 = t27 * t9151;
    let t75611 = t69609 * t16064 * t75609;
    (t75596, t75598, t75602, t75607, t75611)
}
