//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 656/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk656(t32: f64, t4715: f64, t5: f64, t1449: f64, t68: f64, t63: f64, t1435: f64, t437: f64, t1453: f64, t1683: f64, t3: f64, t40: f64) -> (f64, f64, f64, f64, f64) {
    let t4720 = t5 * t4715 * t32;
    let t4721 = 0.34450798614814814813e-2_f64 * t4720;
    let t4726 = 1.0_f64 / t1449 / t68;
    let t4727 = t63 * t4726;
    let t4728 = t1435 * t437;
    let t4729 = t4728 * t1453;
    let t4732 = t1683 * t3;
    let t4733 = t4732 * t40;
    (t4721, t4727, t4728, t4729, t4733)
}
