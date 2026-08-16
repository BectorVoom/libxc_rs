//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2366/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2366(t1851: f64, t8119: f64, t103103: f64, t105102: f64, t105115: f64, t105128: f64, t1396: f64, t1398: f64, t1404: f64, t1852: f64, t20149: f64, t2174: f64, t27930: f64, t29866: f64, t29884: f64, t3: f64, t5364: f64, t580: f64, t6483: f64, t7416: f64, t96281: f64, t96283: f64, t96285: f64) -> f64 {
    let t105131 = t1851 * t8119;
    let t105139 = t103103 + t20149 * t2174 + 2.0_f64 * t5364 * t8119 + t29866 * t1404 + t96281 + t1398 * (t105115 + t105128) + 2.0_f64 * t105131 + t96283 + t7416 * t6483 + t3 * t105102 * t580 + 2.0_f64 * t1852 * t27930 + t1396 * t29884 + t96285;
    t105139
}
