//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 758/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk758(t7896: f64, t8085: f64, t157: f64, t2152: f64, t633: f64, t929: f64, t2176: f64, t880: f64, t639: f64, t7924: f64, t2217: f64, t309: f64) -> (f64, f64, f64, f64, f64) {
    let t8087 = 0.34694512752820797848e1_f64 * t7896 * t8085;
    let t8092 = t2152 * t633 * t929 * t157;
    let t8096 = 0.65854491829355115987e0_f64 * t2176 * t880;
    let t8098 = 0.8673628188205199462e0_f64 * t7924 * t639;
    let t8099 = t2217 * t309;
    (t8087, t8092, t8096, t8098, t8099)
}
