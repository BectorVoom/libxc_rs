//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 198/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk198(t567: f64, t681: f64, t201: f64, t197: f64, t179: f64, t182: f64, t192: f64, t205: f64, t578: f64, t582: f64, t585: f64, t590: f64, t591: f64, t596: f64, t600: f64, t605: f64, t608: f64, t613: f64, t620: f64, t625: f64, t629: f64, t634: f64, t637: f64, t642: f64, t649: f64, t656: f64, t659: f64, t664: f64, t669: f64, t677: f64) -> (f64, f64, f64, f64) {
    let t682 = t681 * t567;
    let t683 = t201 * t682;
    let t684 = t197 * t683;
    let t687 = 0.13900948042322754167e-2_f64 * t578 * t182 - 0.13900948042322754167e-2_f64 * t582 * t585 - 0.34752370105806885418e-4_f64 * t590 * t591 + 0.61789714048124642274e-4_f64 * t596 * t600 - 0.3243554543208642639e-2_f64 * t179 * t605 + 0.13900948042322754167e-2_f64 * t179 * t608 + 0.20272215895054016493e-3_f64 * t613 * t620 - 0.13900948042322754167e-2_f64 * t179 * t625 - 0.57970906942607043474e-5_f64 * t629 * t205 + 0.57970906942607043474e-5_f64 * t634 * t637 + 0.96618178237678405792e-7_f64 * t642 * t649 - 0.1717871209065922055e-6_f64 * t656 * t649 + 0.27053089906549953621e-4_f64 * t192 * t659 - 0.11594181388521408695e-4_f64 * t192 * t664 - 0.16908181191593721013e-5_f64 * t669 * t677 + 0.11594181388521408695e-4_f64 * t192 * t684;
    (t682, t683, t684, t687)
}
