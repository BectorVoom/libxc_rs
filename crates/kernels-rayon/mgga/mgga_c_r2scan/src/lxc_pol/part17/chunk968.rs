//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 968/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk968(t11717: f64, t6085: f64, t10760: f64, t7605: f64, t6093: f64, t8081: f64, t261: f64, t2730: f64, t3304: f64, t10743: f64, t924: f64, t2699: f64, t3290: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11718 = t6085 * t11717;
    let t11720 = t10760 * t7605;
    let t11721 = t6093 * t11720;
    let t11724 = t10760 * t8081;
    let t11725 = t6085 * t11724;
    let t11727 = t261 * t2730;
    let t11728 = t3304 * t11727;
    let t11730 = t10743 * t924;
    let t11732 = t3290 * t2699;
    (t11718, t11720, t11721, t11724, t11725, t11727, t11728, t11730, t11732)
}
