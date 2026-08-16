//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 800/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk800(t7930: f64, t179: f64, t2405: f64, t3026: f64, t404: f64, t1227: f64, t931: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8225 = 0.34246666666666666666e-1_f64 * t7930;
    let t8233 = 0.35616666666666666666e-1_f64 * t7930;
    let t8245 = t179 * t2405 * t3026;
    let t8247 = 0.57165357490759649296e-3_f64 * t404 * t8245;
    let t8253 = t931 * t1227;
    let t8254 = t300 * t8253;
    (t8225, t8233, t8245, t8247, t8253, t8254)
}
