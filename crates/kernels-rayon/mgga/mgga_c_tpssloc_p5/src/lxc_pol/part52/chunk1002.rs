//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1002/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1002(t4300: f64, t6571: f64, t6553: f64, t1880: f64, t1902: f64, t4142: f64, t1492: f64, t6624: f64, t1519: f64, t214: f64, t6572: f64, t13053: f64, t1528: f64, t1912: f64, t23235: f64, t23281: f64, t25200: f64, t25206: f64, t25209: f64, t25211: f64, t25214: f64, t259: f64, t2713: f64, t7538: f64, t855: f64) -> (f64, f64, f64, f64, f64) {
    let t25216 = t6571 * t4300;
    let t25217 = t6553 * t25216;
    let t25218 = t1880 * t25217;
    let t25220 = t4142 * t1902;
    let t25222 = t1492 * t6624;
    let t25224 = t214 * t1519;
    let t25225 = t25224 * t6572;
    let t25226 = t1880 * t25225;
    let t25228 = 0.19190897446562641759e-1_f64 * t23235 + 2.0_f64 * t855 * t25200 - t2713 * t7538 - t23281 * t1528 + 0.41123351671205660912e-2_f64 * t25206 - t13053 * t1912 + 0.38381794893125283518e-1_f64 * t25209 + 0.19190897446562641759e-1_f64 * t25211 - 0.82246703342411321825e-2_f64 * t25214 - 0.82246703342411321825e-2_f64 * t25218 + t25220 * t259 + t25222 * t259 - 0.82246703342411321825e-2_f64 * t25226;
    (t25216, t25220, t25222, t25224, t25228)
}
