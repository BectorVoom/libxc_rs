//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1287/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1287<F: Float>(t12283: F, t20450: F, t20595: F, t68: F, t1340: F, t20556: F, t3799: F, t20570: F, t1362: F, t20512: F, t40021: F, t16288: F, t6422: F) -> (F, F, F, F, F, F, F, F) {
    let t74276 = t12283 * t20450;
    let t74289 = t20595 * t68;
    let t74290 = t74289 * t1340;
    let t74297 = t3799 * t20556;
    let t74299 = t3799 * t20570;
    let t74311 = t74289 * t1362;
    let t74360 = t40021 * t20512;
    let t74376 = t16288 * t6422;
    (t74276, t74289, t74290, t74297, t74299, t74311, t74360, t74376)
}
