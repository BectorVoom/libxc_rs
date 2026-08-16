//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1060/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1060(t1319: f64, t188: f64, t21979: f64, t2229: f64, t3563: f64, t1245: f64, t6316: f64, t6319: f64, t1281: f64, t6976: f64, t1287: f64, t23065: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28626 = t188 * t21979 * t1319;
    let t28628 = t2229 * t3563;
    let t28635 = t6316 * t1245;
    let t28637 = t6319 * t1245;
    let t28700 = t1281 * t6976;
    let t29117 = t23065 * t1287;
    (t28626, t28628, t28635, t28637, t28700, t29117)
}
