//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1013/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1013(t1982: f64, t7428: f64, t8602: f64, t8608: f64, t2139: f64, t27: f64, t3118: f64, t558: f64, t36634: f64, t40972: f64, t40975: f64, t7192: f64) -> (f64, f64, f64, f64, f64) {
    let t42177 = t8602 * t7428 * t1982;
    let t42180 = t8608 * t7428 * t1982;
    let t42196 = t2139 * t27 * t3118 * t558;
    let t42199 = t36634 * t40972;
    let t42201 = t7192 * t40975;
    (t42177, t42180, t42196, t42199, t42201)
}
