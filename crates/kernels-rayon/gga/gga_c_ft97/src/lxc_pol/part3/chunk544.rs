//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 544/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk544(t332: f64, t4317: f64, t1258: f64, t5: f64, t1263: f64, t2253: f64, t327: f64, t703: f64, t3691: f64, t1091: f64, t2923: f64, t904: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4318 = t4317 * t332;
    let t4322 = t5 * t1258;
    let t4332 = t2253 * t1263;
    let t4334 = t703 * t327;
    let t4335 = t4334 * t3691;
    let t4339 = t2923 * t1091 * t904;
    (t4318, t4322, t4332, t4334, t4335, t4339)
}
