//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 863/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk863(t2531: f64, t2567: f64, t360: f64, t2573: f64, t8820: f64, t2551: f64, t2572: f64, t495: f64, t2124: f64, t2550: f64, t8837: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9135 = t2567 * t2531;
    let t9136 = t360 * t9135;
    let t9139 = t8820 * t2573;
    let t9140 = t360 * t9139;
    let t9143 = t8820 * t2551;
    let t9144 = t360 * t9143;
    let t9147 = t2572 * t2531;
    let t9148 = t360 * t9147;
    let t9151 = t8820 * t495;
    let t9152 = t360 * t9151;
    let t9156 = t2124 * t2550 * t2531;
    let t9160 = t2124 * t8837 * t495;
    (t9135, t9136, t9139, t9140, t9143, t9144, t9147, t9148, t9151, t9152, t9156, t9160)
}
