//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 933/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk933(t28800: f64, t7311: f64, t7310: f64, t17983: f64, t9090: f64, t7337: f64, t9030: f64, t28314: f64, t641: f64, t746: f64, t741: f64, t2590: f64, t9078: f64) -> (f64, f64, f64, f64, f64) {
    let t29553 = t7311 * t28800;
    let t29554 = t7310 * t29553;
    let t29556 = t17983 * t9090;
    let t29558 = t7337 * t9030;
    let t29560 = t641 * t28314;
    let t29561 = t746 * t29560;
    let t29562 = t741 * t29561;
    let t29564 = t9078 * t2590;
    (t29554, t29556, t29558, t29562, t29564)
}
