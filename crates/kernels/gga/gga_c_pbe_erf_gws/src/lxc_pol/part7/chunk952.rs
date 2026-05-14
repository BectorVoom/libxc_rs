//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 952/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk952<F: Float>(t19190: F, t5652: F, t1570: F, t481: F, t5651: F, t1368: F, t1457: F, t285: F, t4576: F, t762: F, t159: F, t4562: F, t532: F, t545: F, t5676: F, t102: F, t1533: F, t1544: F) -> (F, F, F, F, F, F, F) {
    let t19191 = t19190 * t5652;
    let t19195 = t5651 * t1570 * t481;
    let t19199 = t1457 * t1368 * t285;
    let t19203 = 0.11622696607154767747e-2 * t762 * t4576 * t285;
    let t19206 = t532 * t4562 * t159 * t285;
    let t19209 = t5676 * t545 * t285;
    let t19216 = 0.1052289e3 * t102 * t1544 * t1533;
    (t19191, t19195, t19199, t19203, t19206, t19209, t19216)
}
