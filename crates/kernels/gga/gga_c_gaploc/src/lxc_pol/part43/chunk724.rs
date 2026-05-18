//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 724/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk724<F: Float>(t14450: F, t14452: F, t209: F, t13762: F, t13764: F, t13766: F, t12870: F, t12873: F, t12877: F, t12880: F, t12883: F, t12889: F, t12893: F, t12896: F, t12902: F, t12909: F, t12911: F, t12921: F) -> (F, F, F, F, F, F) {
    let t14453 = t14450 + t14452;
    let t14454 = t14453 * t209;
    let t14455 = F::new(2.0) * t13762;
    let t14456 = F::new(2.0) * t13764;
    let t14457 = F::new(4.0) * t13766;
    let t14458 = t12870 - t12873 + t12877 - t12880 - t12883 - t12889 + t12893 - t12896 + t12902 + t12909 + t12911 - t12921;
    (t14453, t14454, t14455, t14456, t14457, t14458)
}
