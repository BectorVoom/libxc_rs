//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1145/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1145(t1369: f64, t148132: f64, t28: f64, t9236: f64, t148451: f64, t2112: f64, t34854: f64, t376: f64, t148475: f64, t27072: f64, t5899: f64, t139212: f64, t139213: f64, t139214: f64, t27123: f64) -> (f64, f64, f64, f64, f64) {
    let t148527 = t1369 * t28 * t9236 * t148132;
    let t148530 = t1369 * t28 * t2112 * t148451;
    let t148533 = t1369 * t376 * t34854;
    let t148536 = t5899 * t27072 * t148475;
    let t148540 = t139212 * t139213 * t139214 * t27123;
    (t148527, t148530, t148533, t148536, t148540)
}
