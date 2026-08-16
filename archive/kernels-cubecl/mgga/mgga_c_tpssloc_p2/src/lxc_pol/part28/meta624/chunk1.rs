//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1949/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1949<F: Float>(t16308: F, t22833: F, t16123: F, t2002: F, t559: F, t1307: F, t1377: F, t22633: F, t22635: F, t5353: F, t26215: F, t80650: F) -> (F, F, F, F) {
    let t91413 = t22833 * t16308;
    let t91416 = t16123 * t2002 * t559;
    let t91449 = t22633 * t22635 * t1377 * t5353 * t1307;
    let t91455 = t22633 * t80650 * t26215;
    (t91413, t91416, t91449, t91455)
}
