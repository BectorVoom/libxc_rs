//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1021/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1021(t11260: f64, t852: f64, t833: f64, t11233: f64, t6201: f64, t6199: f64, t11166: f64, t2281: f64, t11155: f64, t11185: f64, t11187: f64, t11191: f64, t11196: f64, t11198: f64, t11200: f64, t11207: f64, t11211: f64, t6249: f64, t6256: f64, t7950: f64, t7955: f64, t9782: f64, t9819: f64, t9826: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11261 = t11260 * t852;
    let t11263 = 1.0_f64 * t833 * t11261;
    let t11264 = t11233 * t6201;
    let t11266 = 0.51726012919273400301e3_f64 * t6199 * t11264;
    let t11269 = t11166 * t2281;
    let t11286 = 0.264729375e1_f64 * t11185 - 0.52945875e1_f64 * t11187 + 0.3529725e1_f64 * t11191 - t6249 + 0.20659e1_f64 * t7955 - 0.1549425e1_f64 * t9782 + 0.1549425e1_f64 * t11155 - 0.157790625e0_f64 * t11196 + 0.94674375e0_f64 * t11198 + 0.6311625e0_f64 * t11200 - t6256 + 0.104195e1_f64 * t7950 - 0.62517e0_f64 * t9819 - 0.62517e0_f64 * t9826 + 0.937755e0_f64 * t11207 + 0.312585e0_f64 * t11211;
    (t11261, t11263, t11264, t11266, t11269, t11286)
}
