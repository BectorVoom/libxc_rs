//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1353/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1353<F: Float>(t14630: F, t2859: F, t888: F, t2877: F, t6866: F, t6773: F, t2437: F, t8072: F, t10144: F, t4614: F, t597: F, t1: F, t10215: F, t106: F, t192: F) -> (F, F, F, F, F, F) {
    let t34119 = F::new(0.23833659967900284447e0) * t2859 * t14630 * t888;
    let t34121 = F::new(0.35750489951850426669e0) * t6866 * t2877;
    let t34123 = F::new(0.71500979903700853338e0) * t6773 * t2877;
    let t34125 = F::new(0.71500979903700853338e0) * t2437 * t8072;
    let t34128 = F::new(0.30674340763136599742e2) * t597 * t4614 * t10144;
    let t34131 = t10215 * t1 * t106 * t192;
    (t34119, t34121, t34123, t34125, t34128, t34131)
}
