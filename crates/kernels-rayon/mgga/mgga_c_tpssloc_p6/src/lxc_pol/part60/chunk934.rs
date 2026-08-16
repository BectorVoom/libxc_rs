//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 934/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk934(t131: f64, t2108: f64, t39063: f64, t8662: f64, t31863: f64, t9239: f64, t22573: f64, t8689: f64, t63: f64, t8308: f64, t113875: f64, t625: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t116065 = t2108 * t131;
    let t116075 = t39063 * t8662;
    let t116106 = t9239 * t31863;
    let t116114 = t8662 * t131;
    let t116115 = t9239 * t116114;
    let t116135 = t8689 * t22573;
    let t117447 = t8308 * t63;
    let t117451 = t113875 * t63;
    let t117477 = t116065 * t117447;
    let t117480 = t79 * t625;
    (t116075, t116106, t116115, t116135, t117447, t117451, t117477, t117480)
}
