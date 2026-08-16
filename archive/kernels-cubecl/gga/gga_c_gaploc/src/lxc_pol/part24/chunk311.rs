//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 311/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk311<F: Float>(t1338: F, t1340: F, t1329: F, t203: F, t492: F, t424: F, t481: F, t482: F) -> (F, F, F, F) {
    let t1341 = t1338 * t1340;
    let t1344 = t1329 * t203;
    let t1345 = t492 * t1344;
    let t1349 = t481 * t482 * t424;
    (t1341, t1344, t1345, t1349)
}
