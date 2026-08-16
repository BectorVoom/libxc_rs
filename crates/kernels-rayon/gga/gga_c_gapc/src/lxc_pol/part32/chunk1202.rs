//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1202/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1202(t34503: f64, t9256: f64, t26007: f64, t3708: f64, t9304: f64, t11455: f64, t9325: f64, t11312: f64, t4940: f64, t11320: f64, t1875: f64, t5190: f64) -> (f64, f64, f64, f64, f64) {
    let t34846 = t34503 * t9256;
    let t34849 = t9304 * t3708 * t26007;
    let t34851 = t11455 * t9325;
    let t34853 = t11312 * t4940;
    let t34856 = t1875 * t11320 * t5190;
    (t34846, t34849, t34851, t34853, t34856)
}
