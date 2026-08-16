//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1479/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1479(t10662: f64, t913: f64, t2842: f64, t2860: f64, t919: f64, t2862: f64, t931: f64) -> (f64, f64, f64, f64) {
    let t10737 = t10662 * t913;
    let t10739 = 6.0_f64 * t2842 * t10737;
    let t10740 = t919 * t2860;
    let t10743 = t2862 * t931;
    (t10737, t10739, t10740, t10743)
}
