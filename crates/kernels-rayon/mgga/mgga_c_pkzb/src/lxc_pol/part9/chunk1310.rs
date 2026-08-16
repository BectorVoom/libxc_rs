//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1310/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1310(t3174: f64, t68: f64, t8277: f64, t1167: f64, t6460: f64, t8435: f64, t8437: f64, t926: f64, t1228: f64, t300: f64, t2387: f64, t919: f64) -> (f64, f64, f64, f64, f64) {
    let t23020 = t3174 * t68 * t8277;
    let t23022 = t1167 * t6460;
    let t23028 = t8435 * t926 * t8437;
    let t23054 = t300 * t1228;
    let t23055 = t2387 * t919;
    (t23020, t23022, t23028, t23054, t23055)
}
