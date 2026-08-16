//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 577/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk577(t22632: f64, t5829: f64, t5830: f64, t22643: f64, t5824: f64, t6: f64, t8907: f64, t8: f64, t3392: f64, t5821: f64, t5813: f64, t5814: f64) -> (f64, f64, f64, f64, f64) {
    let t23766 = t5829 * t22632 * t5830;
    let t23770 = t5824 * t22643;
    let t23772 = t8907 * t6;
    let t23773 = t23772 * t8;
    let t23774 = t3392 * t23773;
    let t23781 = t5821 * t22643;
    let t23789 = t5813 * t22632 * t5814;
    (t23766, t23770, t23774, t23781, t23789)
}
