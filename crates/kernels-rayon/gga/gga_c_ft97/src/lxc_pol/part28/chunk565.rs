//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 565/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk565(t165: f64, t5842: f64, t5765: f64, t92: f64, t1882: f64, t5949: f64, t5958: f64, t1378: f64, t2101: f64) -> (f64, f64, f64, f64, f64) {
    let t23408 = t5842 * t165;
    let t23413 = t5765 * t92;
    let t23425 = t1882 * t5949;
    let t23427 = t1882 * t5958;
    let t23443 = t2101 * t1378;
    (t23408, t23413, t23425, t23427, t23443)
}
