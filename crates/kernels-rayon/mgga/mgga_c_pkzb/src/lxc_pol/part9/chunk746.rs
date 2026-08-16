//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 746/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk746(t1708: f64, t5221: f64, t614: f64, t95: f64, t149: f64, t50: f64, t5181: f64, t581: f64, t164: f64, t1753: f64, t179: f64, t568: f64) -> (f64, f64, f64, f64, f64) {
    let t5222 = t5221 * t1708;
    let t5224 = t95 * t614;
    let t5225 = t149 * t5224;
    let t5227 = t581 * t50 * t5181;
    let t5230 = t1753 * t164;
    let t5232 = t179 * t5230 * t568;
    (t5222, t5224, t5225, t5227, t5232)
}
