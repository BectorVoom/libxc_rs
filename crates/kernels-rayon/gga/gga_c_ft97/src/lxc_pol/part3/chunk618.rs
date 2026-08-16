//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 618/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk618(t2681: f64, t5225: f64, t27: f64, t89: f64, t1196: f64, t284: f64, t291: f64, t1197: f64, t4092: f64, t1208: f64, t4064: f64, t2697: f64, t4939: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5226 = t2681 * t5225;
    let t5228 = t89 * t27 * t5226;
    let t5230 = t1196 * t1196;
    let t5231 = t5230 * t284;
    let t5232 = t5231 * t291;
    let t5234 = t4092 * t1197;
    let t5239 = t4064 * t1208;
    let t5242 = t2697 * t4939;
    (t5226, t5228, t5231, t5232, t5234, t5239, t5242)
}
