//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 801/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk801<F: Float>(t2365: F, t32357: F, t6111: F, t32436: F, t24501: F, t825: F, t9438: F, t33360: F, t787: F, t9824: F, t33348: F, t13141: F, t2464: F, t2684: F) -> (F, F, F, F, F, F) {
    let t43467 = t6111 * t2365 * t32357;
    let t43470 = t6111 * t2365 * t32436;
    let t43476 = t825 * t9438 * t24501;
    let t43522 = t787 * t33360 * t9824;
    let t43526 = t787 * t33348 * t9824;
    let t43581 = t2684 * t2464 * t13141;
    (t43467, t43470, t43476, t43522, t43526, t43581)
}
