//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1996/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1996(t13228: f64, t828: f64, t13223: f64, t232: f64, t253: f64, t254: f64, t1530: f64, t776: f64, t868: f64, t1022: f64, t1409: f64, t382: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25093 = t13228 * t828;
    let t25115 = t13223 * t232;
    let t25168 = t253 * t254;
    let t25365 = t1530 * t776;
    let t25374 = t1530 * t868;
    let t25548 = t1409 * t1022;
    let t25757 = t382 * t254;
    (t25093, t25115, t25168, t25365, t25374, t25548, t25757)
}
