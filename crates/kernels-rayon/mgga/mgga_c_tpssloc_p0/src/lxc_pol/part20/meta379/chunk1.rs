//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1735/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1735(t13160: f64, t776: f64, t2553: f64, t4226: f64, t12971: f64, t824: f64, t13141: f64, t13151: f64, t13157: f64, t1504: f64, t1506: f64, t228: f64, t230: f64, t2667: f64, t2672: f64, t2675: f64, t4219: f64, t4225: f64, t4227: f64, t4230: f64, t822: f64, t825: f64) -> (f64, f64, f64, f64) {
    let t13161 = t13160 * t776;
    let t13164 = t4226 * t2553;
    let t13167 = t824 * t12971;
    let t13170 = -t13141 * t230 - 24.0_f64 * t13151 * t4227 + 60.0_f64 * t13157 * t4225 - 24.0_f64 * t13161 * t4225 - 12.0_f64 * t13164 * t4225 + 3.0_f64 * t13167 * t228 - 12.0_f64 * t1504 * t2672 + 3.0_f64 * t1504 * t2675 + 3.0_f64 * t1506 * t2667 + 6.0_f64 * t4219 * t825 + 6.0_f64 * t4230 * t822;
    (t13161, t13164, t13167, t13170)
}
