//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 664/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk664<F: Float>(t693: F, t226: F, t2428: F, t709: F, t224: F, t2384: F, t2387: F, t2426: F, t2455: F, t3723: F, t3789: F, t678: F, t680: F, t695: F, t807: F, t9524: F, t9525: F, t9530: F, t9533: F, t9535: F, t9539: F, t9543: F, t9545: F, t9548: F, t9601: F, t9609: F, t9614: F, t9618: F, t9622: F, t9625: F, t9631: F, t9677: F) -> (F, F, F, F, F) {
    let t9680 = t693 * t693;
    let t9681 = 1.0 / t9680;
    let t9682 = t226 * t9681;
    let t9683 = t2428 * t709;
    let t9691 = -0.40559281352147498558e-4 * t9524 * t9525 * t2384 + 0.20279640676073749279e-3 * t3723 * t9530 - 0.69764702839313376e-1 * t9533 * t9535 - 0.69764702839313376e-2 * t2387 * t9539 - 0.20279640676073749279e-3 * t9543 * t9545 - 0.32253953169881963531e-5 * t678 * t807 * t9548 - 0.11619434043764639964e-3 * t678 * t9524 * t9548 - 0.11627450473218896e-1 * t678 * t680 * t9601 - 0.279058811357253504e-2 * t678 * t9609 * t9548 + 0.69764702839313376e-2 * t678 * t9614 + 0.34882351419656688e-1 * t2387 * t9618 + 0.34882351419656688e-1 * t2387 * t9622 - 0.58097170218823199822e-3 * t2387 * t9625 - 0.33776098467676728323e-5 * t807 * t9525 * t2384 + 0.58097170218823199823e-3 * t678 * t9631 - t224 * t695 * t9677 - 6.0 * t224 * t9682 * t9683 + 6.0 * t3789 * t2426 * t709 * t2455;
    (t9680, t9681, t9682, t9683, t9691)
}
