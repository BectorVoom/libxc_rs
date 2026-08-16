//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1038/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1038(t2395: f64, t2455: f64, t190: f64, t37991: f64, t195: f64, t25: f64, t2382: f64, t9525: f64, t9524: f64, t2379: f64, t2384: f64, t2387: f64, t2394: f64, t2417: f64, t2418: f64, t2428: f64, t2710: f64, t3723: f64, t3724: f64, t3789: f64, t41573: f64, t41601: f64, t41621: f64, t41622: f64, t41623: f64, t678: f64, t680: f64, t694: f64, t709: f64, t9533: f64, t9545: f64, t9681: f64) -> (f64, f64, f64, f64, f64) {
    let t41652 = t2395 * t2455;
    let t41670 = 96.0_f64 * t37991 * t190;
    let t41673 = t25 / t195 / t41670;
    let t41677 = t9525 * t2382;
    let t41678 = t9524 * t41677;
    let t41681 = 0.38704743803858356237e-5_f64 * t678 * t2710 * t41623 - 36.0_f64 * t3789 * t9681 * t2428 * t2455 - 0.81118562704294997116e-3_f64 * t3723 * t3724 * t694 * t2417 * t709 + 0.46477736175058559857e-3_f64 * t2387 * t9524 * t41573 - 0.139529405678626752e-1_f64 * t2387 * t2394 * t41652 - 0.139529405678626752e0_f64 * t9533 * t680 * t2418 * t2428 + 0.23238868087529279928e-2_f64 * t9533 * t2379 * t41601 + 0.40531318161212073987e-5_f64 * t2710 * t41622 * t2384 + 0.73006706433865497404e-4_f64 * t41621 * t41622 * t2384 + 0.6139293849859577088e-2_f64 * t678 * t41673 * t41623 + 0.16223712540858999423e-3_f64 * t41678 * t9545;
    (t41652, t41670, t41673, t41677, t41681)
}
