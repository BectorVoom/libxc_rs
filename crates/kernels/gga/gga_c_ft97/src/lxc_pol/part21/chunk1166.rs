//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1166/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1166<F: Float>(t116438: F, t446: F, t7824: F, t4436: F, t5617: F, t1317: F, t28: F, t8270: F, t116342: F, t7793: F, t29569: F, t469: F, t473: F, t5665: F, t432: F, t1800: F) -> (F, F, F, F, F, F, F) {
    let t116593 = t446 * t7824 * t116438;
    let t116595 = t5617 * t4436;
    let t116598 = t1317 * t28 * t8270 * t116595;
    let t116601 = t446 * t7793 * t116342;
    let t116606 = t5665 * t28 * t469 * t29569 * t473;
    let t116608 = t29569 * t432;
    let t116611 = t1317 * t28 * t1800 * t116608;
    (t116593, t116595, t116598, t116601, t116606, t116608, t116611)
}
