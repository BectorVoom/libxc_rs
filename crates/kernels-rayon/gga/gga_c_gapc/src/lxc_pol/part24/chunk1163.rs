//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1163/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1163(t128: f64, t3141: f64, t33655: f64, t5462: f64, t623: f64, t11320: f64, t11322: f64, t1932: f64, t11321: f64, t4925: f64, t8950: f64, t11508: f64, t1749: f64, t3060: f64) -> (f64, f64, f64, f64) {
    let t34454 = t5462 * t33655 * t3141 * t623 * t128;
    let t34457 = t1932 * t11320 * t11322;
    let t34460 = t11321 * t4925 * t8950;
    let t34463 = t3060 * t11508 * t1749;
    (t34454, t34457, t34460, t34463)
}
