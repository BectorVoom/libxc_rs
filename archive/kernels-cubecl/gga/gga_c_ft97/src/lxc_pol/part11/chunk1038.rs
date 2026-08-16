//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1038/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1038<F: Float>(t2395: F, t2455: F, t190: F, t37991: F, t195: F, t25: F, t2382: F, t9525: F, t9524: F, t2379: F, t2384: F, t2387: F, t2394: F, t2417: F, t2418: F, t2428: F, t2710: F, t3723: F, t3724: F, t3789: F, t41573: F, t41601: F, t41621: F, t41622: F, t41623: F, t678: F, t680: F, t694: F, t709: F, t9533: F, t9545: F, t9681: F) -> (F, F, F, F, F) {
    let t41652 = t2395 * t2455;
    let t41670 = F::cast_from(96.0_f64) * t37991 * t190;
    let t41673 = t25 / t195 / t41670;
    let t41677 = t9525 * t2382;
    let t41678 = t9524 * t41677;
    let t41681 = F::cast_from(0.38704743803858356237e-5_f64) * t678 * t2710 * t41623 - F::cast_from(36.0_f64) * t3789 * t9681 * t2428 * t2455 - F::cast_from(0.81118562704294997116e-3_f64) * t3723 * t3724 * t694 * t2417 * t709 + F::cast_from(0.46477736175058559857e-3_f64) * t2387 * t9524 * t41573 - F::cast_from(0.139529405678626752e-1_f64) * t2387 * t2394 * t41652 - F::cast_from(0.139529405678626752e0_f64) * t9533 * t680 * t2418 * t2428 + F::cast_from(0.23238868087529279928e-2_f64) * t9533 * t2379 * t41601 + F::cast_from(0.40531318161212073987e-5_f64) * t2710 * t41622 * t2384 + F::cast_from(0.73006706433865497404e-4_f64) * t41621 * t41622 * t2384 + F::cast_from(0.6139293849859577088e-2_f64) * t678 * t41673 * t41623 + F::cast_from(0.16223712540858999423e-3_f64) * t41678 * t9545;
    (t41652, t41670, t41673, t41677, t41681)
}
