//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1284/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1284(t16426: f64, t3786: f64, t1444: f64, t1897: f64, t2642: f64, t3761: f64, t1098: f64, t5483: f64, t1517: f64, t531: f64, t545: f64, t1992: f64, t3251: f64) -> (f64, f64, f64, f64, f64) {
    let t16427 = t3786 * t16426;
    let t16432 = t3761 * t1897 * t1444 * t2642;
    let t16436 = 0.19711289e-2_f64 * t1098 * t5483;
    let t16438 = t1517 * t545 * t531;
    let t16441 = t3251 * t1992;
    (t16427, t16432, t16436, t16438, t16441)
}
