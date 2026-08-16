//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1010/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1010(t1286: f64, t34361: f64, t376: f64, t46727: f64, t7274: f64, t137350: f64, t137353: f64, t137354: f64, t25539: f64, t25584: f64, t25849: f64, t32054: f64, t34562: f64, t38652: f64, t492: f64, t5743: f64, t6423: f64, t6457: f64, t6547: f64, t7162: f64, t7214: f64, t8418: f64) -> (f64, f64) {
    let t144687 = t1286 * t376 * t34361;
    let t144701 = t46727 * t7274;
    let t144703 = t7162 * t25539 / 6.0_f64 + t7162 * t25849 / 6.0_f64 - t32054 * t6423 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t144687 - t137350 / 18.0_f64 + t137353 - t137354 / 18.0_f64 + t32054 * t6457 / 6.0_f64 + t25584 * t7214 / 6.0_f64 + 48.0_f64 * t38652 * t34562 * t492 - 24.0_f64 * t8418 * t6547 * t5743 + 4.0_f64 * t144701;
    (t144701, t144703)
}
