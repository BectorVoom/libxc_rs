//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 824/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk824<F: Float>(t10608: F, t9272: F, t9278: F, t1445: F, t26809: F, t3085: F, t4527: F, t8411: F, t9327: F, t10556: F, t1415: F, t9321: F, t2487: F, t41878: F, t6711: F, t34600: F, t544: F, t9287: F) -> (F, F, F, F, F, F) {
    let t42349 = t9272 * t10608 * t9278;
    let t42350 = 0.11502877786176224903e1 * t42349;
    let t42354 = 0.27606906686822939767e2 * t4527 * t1445 * t26809 * t3085;
    let t42356 = 0.10725146985555128001e1 * t8411 * t9327;
    let t42359 = 0.42900587942220512003e1 * t1415 * t10556 * t9321;
    let t42363 = t2487 * t6711 * t41878;
    let t42366 = t544 * t34600 * t9287;
    (t42350, t42354, t42356, t42359, t42363, t42366)
}
