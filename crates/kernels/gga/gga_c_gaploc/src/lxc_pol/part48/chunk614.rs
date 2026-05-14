//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 614/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk614<F: Float>(t13118: F, t6111: F, t10893: F, t959: F, t10012: F, t1022: F, t9438: F, t2684: F, t2610: F, t3431: F, t2365: F, t2033: F, t10007: F, t825: F, t2558: F, t3464: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13119 = t6111 * t13118;
    let t13121 = t10893 * t959;
    let t13141 = t10012 * t1022;
    let t13142 = t9438 * t13141;
    let t13143 = t2684 * t13142;
    let t13145 = t2610 * t3431;
    let t13146 = t2365 * t13145;
    let t13147 = t2033 * t13146;
    let t13149 = t10007 * t1022;
    let t13150 = t9438 * t13149;
    let t13151 = t825 * t13150;
    let t13176 = t3464 * t2558;
    (t13119, t13121, t13141, t13142, t13143, t13145, t13146, t13147, t13149, t13150, t13151, t13176)
}
