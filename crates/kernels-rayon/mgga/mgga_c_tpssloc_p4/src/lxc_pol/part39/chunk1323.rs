//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1323/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1323(t110972: f64, t111017: f64, t111168: f64, t111213: f64, t112: f64, t30349: f64, t110376: f64, t110926: f64, t111143: f64, t1401: f64, t1458: f64, t16524: f64, t16535: f64, t16538: f64, t2199: f64, t30071: f64, t30109: f64, t30112: f64, t30128: f64, t30315: f64, t3938: f64, t3941: f64, t4072: f64, t45560: f64, t5371: f64, t55341: f64, t55405: f64, t577: f64, t66940: f64, t671: f64, t8189: f64, t8273: f64, t8294: f64) -> (f64, f64) {
    let t111215 = t110972 + t111017 + t111168 + t111213;
    let t111226 = t30349 * t112;
    let t111243 = 27.0_f64 * t3938 * t30315 + 0.135e2_f64 * t55341 * t2199 + 54.0_f64 * t30112 * t16538 + 27.0_f64 * t110926 * t1458 + 27.0_f64 * t16524 * t30128 + 0.45e1_f64 * t111215 * t577 + 27.0_f64 * t3941 * t30071 * t1458 + 54.0_f64 * t3941 * t8189 * t4072 + 27.0_f64 * t45560 * t8294 + 27.0_f64 * t111226 * t671 + 27.0_f64 * t30109 * t4072 + 0.135e2_f64 * t5371 * t30071 + 27.0_f64 * t16535 * t8273 + 54.0_f64 * t66940 * t8294 + 27.0_f64 * t55405 * t2199 + 0.135e2_f64 * t110376 * t1458 + 0.135e2_f64 * t1401 * t111143;
    (t111215, t111243)
}
