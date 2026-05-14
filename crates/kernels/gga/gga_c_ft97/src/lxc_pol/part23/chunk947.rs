//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 947/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk947<F: Float>(t28719: F, t799: F, t27: F, t89: F, t24981: F, t684: F, t7062: F, t24980: F, t856: F, t992: F, t6334: F, t25026: F, t92: F) -> (F, F, F, F, F, F, F, F) {
    let t28720 = t799 * t28719;
    let t28722 = t89 * t27 * t28720;
    let t28726 = t24981 * t7062 * t684;
    let t28727 = t24980 * t28726;
    let t28729 = t992 * t856;
    let t28731 = t24981 * t6334 * t28729;
    let t28732 = t24980 * t28731;
    let t28735 = t25026 * t92;
    (t28720, t28722, t28726, t28727, t28729, t28731, t28732, t28735)
}
