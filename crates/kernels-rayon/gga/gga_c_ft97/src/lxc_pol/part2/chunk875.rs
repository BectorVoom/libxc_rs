//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 875/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk875(t13352: f64, t3806: f64, t701: f64, t3813: f64, t8715: f64, t2917: f64, t668: f64, t228: f64, t9634: f64, t2436: f64, t3799: f64, t2452: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13624 = t3806 * t13352;
    let t13625 = t701 * t13624;
    let t13628 = t8715 * t3813;
    let t13629 = t701 * t13628;
    let t13631 = t2917 * t668;
    let t13633 = t228 * t9634 * t13631;
    let t13635 = t3799 * t2436;
    let t13636 = 0.1134997482304526749e-1_f64 * t13635;
    let t13637 = t3799 * t2452;
    (t13625, t13629, t13633, t13635, t13636, t13637)
}
