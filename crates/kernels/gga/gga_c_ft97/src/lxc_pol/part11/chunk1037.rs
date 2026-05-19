//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1037/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1037<F: Float>(t200: F, t41627: F, t13531: F, t236: F, t2379: F, t2387: F, t2394: F, t2418: F, t2455: F, t3723: F, t3724: F, t41577: F, t41589: F, t41593: F, t41601: F, t41621: F, t41623: F, t678: F, t680: F, t689: F, t9530: F, t9533: F, t9600: F, t9609: F, t9677: F, t9683: F) -> F {
    let t41628 = t41627 * t200;
    let t41635 = F::cast_from(0.93019603785751168e-2_f64) * t678 * t2394 * t41589 + F::cast_from(0.279058811357253504e0_f64) * t41593 * t680 * t689 * t9683 - F::cast_from(0.1674352868143521024e-1_f64) * t678 * t9609 * t41577 + F::cast_from(0.279058811357253504e-1_f64) * t9533 * t2394 * t41601 + F::cast_from(0.69764702839313376e-1_f64) * t2387 * t680 * t2418 * t2455 + F::cast_from(0.46509801892875584e-1_f64) * t2387 * t680 * t689 * t9677 + F::cast_from(0.27039520901431665705e-3_f64) * t3723 * t3724 * t236 * t9600 - F::cast_from(0.20265659080606036993e-4_f64) * t13531 * t9530 + F::cast_from(0.20914981278776351936e-3_f64) * t678 * t41621 * t41623 + F::cast_from(0.69764702839313376e-2_f64) * t678 * t2394 * t41628 + F::cast_from(0.58097170218823199823e-3_f64) * t678 * t2379 * t41628;
    t41635
}
