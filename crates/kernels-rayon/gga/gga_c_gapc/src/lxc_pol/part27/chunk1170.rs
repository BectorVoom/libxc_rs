//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1170/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1170(t144: f64, t3095: f64, t3094: f64, t3954: f64, t128: f64, t3141: f64, t33655: f64, t5462: f64, t623: f64, t11320: f64, t11322: f64, t1932: f64) -> (f64, f64, f64, f64) {
    let t34447 = t3095 * t144;
    let t34449 = t3094 * t34447 * t3954;
    let t34454 = t5462 * t33655 * t3141 * t623 * t128;
    let t34457 = t1932 * t11320 * t11322;
    (t34447, t34449, t34454, t34457)
}
