//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 721/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk721(t2936: f64, t8559: f64, t2952: f64, t4865: f64, t4868: f64, t8362: f64, t1005: f64, t4883: f64, t2937: f64, t4885: f64, t4015: f64, t4018: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8560 = t2936 * t8559;
    let t8562 = t2952 * t4865;
    let t8563 = t8362 * t4868;
    let t8564 = t8562 * t8563;
    let t8566 = t1005 * t4883;
    let t8567 = t2937 * t4885;
    let t8568 = t8566 * t8567;
    let t8570 = t2952 * t4015;
    let t8571 = t8362 * t4018;
    (t8560, t8562, t8564, t8568, t8570, t8571)
}
