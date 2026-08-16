//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 363/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk363(t505: f64, t6135: f64, t2354: f64, t446: f64, t6008: f64, t713: f64, t193: f64, t89: f64, t6061: f64, t676: f64, t27: f64, t6113: f64, t6117: f64, t6122: f64, t6126: f64, t6130: f64, t6134: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6136 = t6135 * t505;
    let t6137 = t2354 * t6136;
    let t6138 = t446 * t6137;
    let t6140 = t6008 * t713;
    let t6141 = t193 * t6140;
    let t6142 = t89 * t6141;
    let t6144 = t676 * t6061;
    let t6146 = t89 * t27 * t6144;
    let t6148 = t6113 / 12.0_f64 + t6117 + t6122 / 18.0_f64 + t6126 / 3.0_f64 - t6130 / 6.0_f64 + t6134 + t6138 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t6142 - t6146 / 3.0_f64;
    (t6136, t6137, t6138, t6140, t6141, t6142, t6144, t6146, t6148)
}
