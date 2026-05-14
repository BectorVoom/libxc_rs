//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 525/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk525<F: Float>(t10677: F, t550: F, t1843: F, t10627: F, t688: F, t779: F, t2508: F, t296: F, t3431: F, t123: F, t734: F, t2554: F, t2932: F, t7064: F, t5539: F, t8769: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10678 = t550 * t10677;
    let t10679 = t1843 * t10678;
    let t10682 = t10627 * t688;
    let t10683 = t779 * t10682;
    let t10685 = 0.76905262301422242837e-2 * t2508 * t10683;
    let t10686 = t296 * t3431;
    let t10687 = t10686 * t123;
    let t10688 = t10687 * t734;
    let t10691 = t2932 * t2554;
    let t10692 = t7064 * t10691;
    let t10693 = 0.32043859292259267849e-3 * t10692;
    let t10694 = t5539 * t8769;
    (t10678, t10679, t10685, t10686, t10687, t10688, t10692, t10693, t10694)
}
