//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 709/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk709(t693: f64, t226: f64, t2428: f64, t709: f64, t224: f64, t2384: f64, t2387: f64, t2426: f64, t2455: f64, t3723: f64, t3789: f64, t678: f64, t680: f64, t695: f64, t807: f64, t9524: f64, t9525: f64, t9530: f64, t9533: f64, t9535: f64, t9539: f64, t9543: f64, t9545: f64, t9548: f64, t9601: f64, t9609: f64, t9614: f64, t9618: f64, t9622: f64, t9625: f64, t9631: f64, t9677: f64) -> (f64, f64, f64, f64, f64) {
    let t9680 = t693 * t693;
    let t9681 = 1.0_f64 / t9680;
    let t9682 = t226 * t9681;
    let t9683 = t2428 * t709;
    let t9691 = -0.40559281352147498558e-4_f64 * t9524 * t9525 * t2384 + 0.20279640676073749279e-3_f64 * t3723 * t9530 - 0.69764702839313376e-1_f64 * t9533 * t9535 - 0.69764702839313376e-2_f64 * t2387 * t9539 - 0.20279640676073749279e-3_f64 * t9543 * t9545 - 0.32253953169881963531e-5_f64 * t678 * t807 * t9548 - 0.11619434043764639964e-3_f64 * t678 * t9524 * t9548 - 0.11627450473218896e-1_f64 * t678 * t680 * t9601 - 0.279058811357253504e-2_f64 * t678 * t9609 * t9548 + 0.69764702839313376e-2_f64 * t678 * t9614 + 0.34882351419656688e-1_f64 * t2387 * t9618 + 0.34882351419656688e-1_f64 * t2387 * t9622 - 0.58097170218823199822e-3_f64 * t2387 * t9625 - 0.33776098467676728323e-5_f64 * t807 * t9525 * t2384 + 0.58097170218823199823e-3_f64 * t678 * t9631 - t224 * t695 * t9677 - 6.0_f64 * t224 * t9682 * t9683 + 6.0_f64 * t3789 * t2426 * t709 * t2455;
    (t9680, t9681, t9682, t9683, t9691)
}
