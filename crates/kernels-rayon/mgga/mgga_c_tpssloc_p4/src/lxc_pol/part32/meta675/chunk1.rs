//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2112/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2112(t1210: f64, t24721: f64, t27691: f64, t27700: f64, t86261: f64, t15418: f64, t2121: f64, t4724: f64, t24720: f64, t27710: f64, t24722: f64, t11588: f64, t4729: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95571 = 0.20186378047070195428e-3_f64 * t24721 * t1210 * t27691;
    let t95573 = 0.20186378047070195428e-3_f64 * t86261 * t27700;
    let t95587 = t2121 * t15418 * t4724 / 324.0_f64;
    let t95588 = t27710 * t24720;
    let t95590 = 0.16149102437656156342e-2_f64 * t95588 * t24722;
    let t95593 = t2121 * t11588 * t4729 / 216.0_f64;
    (t95571, t95573, t95587, t95588, t95590, t95593)
}
