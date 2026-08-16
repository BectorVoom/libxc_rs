//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1269/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1269(t3348: f64, t4999: f64, t14703: f64, t26896: f64, t26917: f64, t28059: f64, t1096: f64, t14800: f64, t8072: f64, t92525: f64, t14833: f64, t92447: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95444 = t4999 * t3348;
    let t95446 = t26896 * t14703;
    let t95448 = t28059 * t26917;
    let t95450 = t1096 * t14800;
    let t95453 = t92525 * t8072;
    let t95455 = t92447 * t14833;
    (t95444, t95446, t95448, t95450, t95453, t95455)
}
