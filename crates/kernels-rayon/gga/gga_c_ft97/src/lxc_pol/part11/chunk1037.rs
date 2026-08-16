//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1037/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1037(t200: f64, t41627: f64, t13531: f64, t236: f64, t2379: f64, t2387: f64, t2394: f64, t2418: f64, t2455: f64, t3723: f64, t3724: f64, t41577: f64, t41589: f64, t41593: f64, t41601: f64, t41621: f64, t41623: f64, t678: f64, t680: f64, t689: f64, t9530: f64, t9533: f64, t9600: f64, t9609: f64, t9677: f64, t9683: f64) -> f64 {
    let t41628 = t41627 * t200;
    let t41635 = 0.93019603785751168e-2_f64 * t678 * t2394 * t41589 + 0.279058811357253504e0_f64 * t41593 * t680 * t689 * t9683 - 0.1674352868143521024e-1_f64 * t678 * t9609 * t41577 + 0.279058811357253504e-1_f64 * t9533 * t2394 * t41601 + 0.69764702839313376e-1_f64 * t2387 * t680 * t2418 * t2455 + 0.46509801892875584e-1_f64 * t2387 * t680 * t689 * t9677 + 0.27039520901431665705e-3_f64 * t3723 * t3724 * t236 * t9600 - 0.20265659080606036993e-4_f64 * t13531 * t9530 + 0.20914981278776351936e-3_f64 * t678 * t41621 * t41623 + 0.69764702839313376e-2_f64 * t678 * t2394 * t41628 + 0.58097170218823199823e-3_f64 * t678 * t2379 * t41628;
    t41635
}
