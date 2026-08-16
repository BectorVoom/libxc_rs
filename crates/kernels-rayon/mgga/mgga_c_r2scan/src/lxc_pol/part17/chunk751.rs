//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 751/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk751(t1607: f64, t5100: f64, t512: f64, t6101: f64, t507: f64, t1591: f64, t2168: f64, t1541: f64, t545: f64, t548: f64, t110: f64, t6189: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6420 = t5100 * t1607;
    let t6422 = t512 * t6101;
    let t6424 = 0.174549769648958674e0_f64 * t6422 * t507;
    let t6425 = t1591 * t2168;
    let t6448 = t545 * t1541;
    let t6449 = t6448 * t548;
    let t6461 = t6189 * t110;
    (t6420, t6424, t6425, t6448, t6449, t6461)
}
