//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2336/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2336(t324: f64, t47740: f64, t47789: f64, t300: f64, t1589: f64, t42281: f64, t10696: f64, t2842: f64, t4399: f64, t10662: f64, t1556: f64, t42100: f64, t42102: f64) -> (f64, f64, f64, f64, f64) {
    let t47791 = (t47740 + t47789) * t324;
    let t47793 = 0.19751673498613801407e-1_f64 * t300 * t47791;
    let t47795 = 0.5848223622634646207e0_f64 * t42281 * t1589;
    let t47798 = 0.16081979498692535067e2_f64 * t2842 * t4399 * t10696;
    let t47802 = 0.24955700379505800916e5_f64 * t42100 * t1556 * t42102 * t10662;
    (t47791, t47793, t47795, t47798, t47802)
}
