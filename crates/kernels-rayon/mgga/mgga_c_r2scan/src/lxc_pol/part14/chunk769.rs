//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 769/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk769(t2078: f64, t784: f64, t783: f64, t788: f64, t1607: f64, t5100: f64, t512: f64, t6101: f64, t507: f64, t1591: f64, t2168: f64, t1584: f64, t1634: f64) -> (f64, f64, f64, f64, f64) {
    let t6416 = t2078 * t784;
    let t6418 = t783 * t6416 * t788;
    let t6420 = t5100 * t1607;
    let t6422 = t512 * t6101;
    let t6424 = 0.174549769648958674e0_f64 * t6422 * t507;
    let t6425 = t1591 * t2168;
    let t6440 = t1584 * t1634;
    (t6418, t6420, t6424, t6425, t6440)
}
