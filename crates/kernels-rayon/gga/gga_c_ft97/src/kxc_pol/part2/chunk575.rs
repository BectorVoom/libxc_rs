//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 575/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk575(t3691: f64, t3806: f64, t701: f64, t2320: f64, t3700: f64, t18: f64, t704: f64, t2248: f64, t2435: f64, t2437: f64, t3796: f64, t3800: f64, t3804: f64) -> (f64, f64, f64, f64, f64) {
    let t3807 = t3806 * t3691;
    let t3808 = t701 * t3807;
    let t3810 = t2320 * t3700;
    let t3811 = t701 * t3810;
    let t3813 = t704 * t18;
    let t3814 = t2248 * t3813;
    let t3815 = t701 * t3814;
    let t3817 = -0.17024962234567901235e-1_f64 * t3796 - 0.17024962234567901235e-1_f64 * t3800 - t2435 + 0.21281202793209876543e-2_f64 * t2437 + 0.21281202793209876543e-2_f64 * t3804 + 0.85124811172839506173e-2_f64 * t3808 - 0.12768721675925925926e-1_f64 * t3811 - 0.12768721675925925926e-1_f64 * t3815;
    (t3808, t3811, t3813, t3815, t3817)
}
