//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1063/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1063(t13440: f64, t9410: f64, t3200: f64, t4801: f64, t9415: f64, t1126: f64, t4924: f64, t303: f64, t2635: f64, t4566: f64) -> (f64, f64, f64, f64) {
    let t13441 = t9410 * t13440;
    let t13442 = t3200 * t13441;
    let t13444 = t9415 * t4801;
    let t13445 = t3200 * t13444;
    let t13447 = t4924 * t1126;
    let t13448 = t303 * t13447;
    let t13462 = t4566 * t2635;
    (t13442, t13445, t13448, t13462)
}
