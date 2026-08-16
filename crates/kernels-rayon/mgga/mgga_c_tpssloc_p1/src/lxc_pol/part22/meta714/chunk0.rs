//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2317/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2317(t21061: f64, t225: f64, t21036: f64, t20856: f64, t252: f64, t1519: f64, t5584: f64, t20852: f64, t13176: f64, t13433: f64, t13453: f64, t16673: f64, t16756: f64, t16758: f64, t16762: f64, t16817: f64, t16825: f64, t16830: f64, t16935: f64, t17034: f64, t21025: f64, t4166: f64, t4182: f64, t4281: f64, t4296: f64, t5612: f64, t5645: f64, t5651: f64, t58313: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67339 = t21061 * t225;
    let t67344 = t21036 * t225;
    let t67350 = t252 * t20856;
    let t67358 = t1519 * t5584;
    let t67392 = t252 * t20852;
    let t67403 = -3.0_f64 * t13433 * t5612 * t812 + 12.0_f64 * t16758 * t16935 * t4281 + 2.0_f64 * t4182 * t4281 * t67392 + 6.0_f64 * t13176 * t5645 - 3.0_f64 * t13176 * t5651 + 6.0_f64 * t13453 * t21025 - 3.0_f64 * t16673 * t4296 - 3.0_f64 * t16756 * t4166 - 6.0_f64 * t16762 * t16830 - 18.0_f64 * t16817 * t58313 + 18.0_f64 * t16825 * t17034;
    (t67339, t67344, t67350, t67358, t67392, t67403)
}
