//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 862/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk862<F: Float>(t3645: F, t443: F, t1004: F, t3102: F, t3062: F, t3077: F, t1160: F, t180: F, t3101: F, t407: F, t12265: F, t150: F) -> (F, F, F, F, F) {
    let t12282 = t3645 * t443;
    let t12285 = F::new(0.26341796731742046395e1) * t1004 * t3102;
    let t12286 = t3077 * t3062;
    let t12290 = t1160 * t180 * t3101 * t407;
    let t12295 = t12265 * t150;
    (t12282, t12285, t12286, t12290, t12295)
}
