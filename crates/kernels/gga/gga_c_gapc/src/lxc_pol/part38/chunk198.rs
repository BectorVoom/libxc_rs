//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 198/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk198<F: Float>(t567: F, t681: F, t201: F, t197: F, t179: F, t182: F, t192: F, t205: F, t578: F, t582: F, t585: F, t590: F, t591: F, t596: F, t600: F, t605: F, t608: F, t613: F, t620: F, t625: F, t629: F, t634: F, t637: F, t642: F, t649: F, t656: F, t659: F, t664: F, t669: F, t677: F) -> (F, F, F, F) {
    let t682 = t681 * t567;
    let t683 = t201 * t682;
    let t684 = t197 * t683;
    let t687 = 0.13900948042322754167e-2 * t578 * t182 - 0.13900948042322754167e-2 * t582 * t585 - 0.34752370105806885418e-4 * t590 * t591 + 0.61789714048124642274e-4 * t596 * t600 - 0.3243554543208642639e-2 * t179 * t605 + 0.13900948042322754167e-2 * t179 * t608 + 0.20272215895054016493e-3 * t613 * t620 - 0.13900948042322754167e-2 * t179 * t625 - 0.57970906942607043474e-5 * t629 * t205 + 0.57970906942607043474e-5 * t634 * t637 + 0.96618178237678405792e-7 * t642 * t649 - 0.1717871209065922055e-6 * t656 * t649 + 0.27053089906549953621e-4 * t192 * t659 - 0.11594181388521408695e-4 * t192 * t664 - 0.16908181191593721013e-5 * t669 * t677 + 0.11594181388521408695e-4 * t192 * t684;
    (t682, t683, t684, t687)
}
