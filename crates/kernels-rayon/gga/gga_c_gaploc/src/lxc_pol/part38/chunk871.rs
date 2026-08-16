//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 871/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk871(t299: f64, t3431: f64, t550: f64, t43027: f64, t13624: f64, t1841: f64, t2536: f64, t734: f64, t1022: f64) -> (f64, f64, f64, f64, f64) {
    let t44878 = t299 * t3431;
    let t44879 = t550 * t44878;
    let t44883 = 0.1281754371690370714e-2_f64 * t43027;
    let t44887 = 0.85450291446024714263e-3_f64 * t1841 * t2536 * t13624 * t734;
    let t44888 = t1022 * t3431;
    (t44878, t44879, t44883, t44887, t44888)
}
