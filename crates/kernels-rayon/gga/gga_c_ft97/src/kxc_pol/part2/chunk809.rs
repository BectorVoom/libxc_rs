//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 809/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk809(t582: f64, t605: f64, t2212: f64, t2992: f64, t2097: f64, t2983: f64, t11982: f64, t3440: f64, t3439: f64, t157: f64, t9224: f64, t160: f64, t7763: f64) -> (f64, f64, f64, f64, f64) {
    let t12709 = t582 * t605;
    let t12710 = t2992 * t2212;
    let t12711 = t12709 * t12710;
    let t12714 = t2097 * t605;
    let t12715 = t2983 * t2212;
    let t12716 = t12714 * t12715;
    let t12719 = t3440 * t11982;
    let t12720 = t3439 * t12719;
    let t12723 = t9224 * t157;
    let t12724 = t160 * t7763;
    (t12711, t12716, t12720, t12723, t12724)
}
