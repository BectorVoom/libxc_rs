//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1075/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1075<F: Float>(t21600: F, t79: F, t534: F, t14925: F, t14940: F, t1593: F, t21555: F, t21558: F, t21561: F, t21567: F, t2328: F, t4502: F, t4510: F, t535: F, t541: F, t6450: F, t6453: F) -> (F,) {
    let t21601 = t79 * t21600;
    let t21602 = t21601 * t534;
    let t21606 = 0.39979530480394038252e-2 * t14925 + 0.14392630972941853771e0 * t6453 * t1593 - t21555 - t21558 + 0.17990788716177317213e-1 * t535 * t21561 + 0.14392630972941853771e0 * t4502 * t2328 - t21567 - 0.26386490117060065246e0 * t4510 * t2328 - 0.5397236614853195164e-1 * t6450 * t1593 + 0.2698618307426597582e-1 * t21602 * t541 - 0.11993859144118211476e-1 * t14940;
    (t21606,)
}
