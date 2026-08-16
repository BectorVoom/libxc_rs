//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1042/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1042(t1105: f64, t353: f64, t3722: f64, t4386: f64, t13635: f64, t2246: f64, t13606: f64, t2376: f64, t829: f64, t830: f64, t11806: f64, t337: f64, t6560: f64) -> (f64, f64, f64, f64) {
    let t44149 = t4386 * t353 * t3722 * t1105;
    let t44158 = t2246 * t13635;
    let t44186 = t2376 * t13606;
    let t44188 = t829 * t830 * t44186;
    let t44213 = t11806 * t1105;
    let t44215 = t6560 * t337 * t44213;
    (t44149, t44158, t44188, t44215)
}
