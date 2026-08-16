//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3061/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3061(t11424: f64, t18680: f64, t14913: f64, t1671: f64, t3264: f64, t18683: f64, t44162: f64, t11190: f64, t3307: f64, t6024: f64, t18265: f64, t3265: f64, t43969: f64) -> (f64, f64, f64, f64, f64) {
    let t63563 = 8.0_f64 * t11424 * t18680;
    let t63566 = 4.0_f64 * t3264 * t1671 * t14913;
    let t63568 = 0.19298375398431042081e3_f64 * t44162 * t18683;
    let t63571 = 0.96491876992155210402e2_f64 * t11190 * t6024 * t3307;
    let t63574 = 0.62071215503128080361e4_f64 * t43969 * t18265 * t3265;
    (t63563, t63566, t63568, t63571, t63574)
}
