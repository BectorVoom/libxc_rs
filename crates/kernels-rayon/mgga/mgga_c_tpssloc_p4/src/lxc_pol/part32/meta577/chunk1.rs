//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1955/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1955(t29535: f64, t3598: f64, t6267: f64, t7301: f64, t7300: f64, t2123: f64, t6140: f64, t1716: f64, t8010: f64, t27382: f64, t2130: f64, t46: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29536 = t3598 * t29535;
    let t29545 = t7301 * t6267;
    let t29546 = t7300 * t29545;
    let t29551 = t6140 * t2123;
    let t29554 = t1716 * t8010;
    let t29557 = t1716 * t27382;
    let t29560 = t2130 * t46;
    (t29536, t29545, t29546, t29551, t29554, t29557, t29560)
}
