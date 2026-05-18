//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1063/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1063<F: Float>(t5628: F, t5660: F, t142: F, t520: F, t5652: F, t1570: F, t481: F, t5651: F, t1368: F, t1457: F, t285: F, t4576: F, t762: F) -> (F, F, F, F, F) {
    let t19187 = t5660 * t5628;
    let t19190 = t520 * t142;
    let t19191 = t19190 * t5652;
    let t19195 = t5651 * t1570 * t481;
    let t19199 = t1457 * t1368 * t285;
    let t19203 = F::new(0.11622696607154767747e-2) * t762 * t4576 * t285;
    (t19187, t19191, t19195, t19199, t19203)
}
