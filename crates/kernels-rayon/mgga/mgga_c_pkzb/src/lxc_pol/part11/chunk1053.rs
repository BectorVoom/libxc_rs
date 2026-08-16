//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1053/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1053(t204: f64, t5026: f64, t5070: f64, t1503: f64, t4952: f64, t5335: f64, t555: f64, t1497: f64, t1622: f64, t4920: f64, t4885: f64, t496: f64) -> (f64, f64, f64, f64) {
    let t16489 = 0.4274e0_f64 * t204 * t5070 * t5026;
    let t16493 = 0.69263436422725855036e2_f64 * t555 * t1503 * t4952 * t5335;
    let t16497 = 0.62337092780453269531e3_f64 * t555 * t4920 * t1497 * t1622;
    let t16502 = t496 * t4885;
    (t16489, t16493, t16497, t16502)
}
