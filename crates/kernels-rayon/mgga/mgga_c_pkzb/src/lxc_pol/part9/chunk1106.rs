//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1106/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1106(t11817: f64, t204: f64, t334: f64, t1731: f64, t218: f64, t344: f64, t5555: f64, t847: f64, t1878: f64, t2226: f64, t2230: f64, t6185: f64, t675: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18439 = t204 * t11817 * t334;
    let t18440 = 0.31310740740740740741e1_f64 * t18439;
    let t18442 = t218 * t1731 * t344;
    let t18443 = 0.13490888888888888889e1_f64 * t18442;
    let t18445 = t218 * t5555 * t847;
    let t18448 = t218 * t1878 * t2226;
    let t18451 = t218 * t1878 * t2230;
    let t18454 = t218 * t675 * t6185;
    (t18439, t18440, t18442, t18443, t18445, t18448, t18451, t18454)
}
