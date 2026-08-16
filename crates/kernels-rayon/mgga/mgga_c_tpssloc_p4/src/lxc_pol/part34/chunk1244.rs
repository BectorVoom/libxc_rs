//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1244/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1244(t102163: f64, t102168: f64, t102187: f64, t102303: f64, t106758: f64, t106800: f64, t106804: f64, t1860: f64, t2031: f64, t2032: f64, t23963: f64, t26016: f64, t26954: f64, t27937: f64, t28935: f64, t7428: f64, t7782: f64, t84280: f64, t90137: f64, t92003: f64, t96473: f64) -> f64 {
    let t108763 = 10.0_f64 * t96473 * t26954 + 20.0_f64 * t26016 * t102163 + 10.0_f64 * t26016 * t102168 + 30.0_f64 * t23963 * t106758 - 60.0_f64 * t90137 * t102187 + t106804 * t2032 / 3.0_f64 + t27937 * t7782 + t7428 * t28935 + t1860 * t2031 * t106800 / 3.0_f64 + 88.0_f64 / 9.0_f64 * t92003 - t84280 - 8.0_f64 / 3.0_f64 * t102303;
    t108763
}
