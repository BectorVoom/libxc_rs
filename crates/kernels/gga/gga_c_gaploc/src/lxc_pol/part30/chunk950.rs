//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 950/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk950<F: Float>(t2086: F, t2109: F, t2102: F, t2154: F, t169: F, t4585: F, t2683: F, t5580: F, t121: F, t5745: F, t2084: F, t321: F, t2088: F, t324: F, t1953: F, t1959: F) -> (F, F, F, F, F, F, F, F) {
    let t16136 = t2109 * t2086;
    let t16239 = t2154 * t2102;
    let t16251 = t4585 * t169;
    let t16455 = t5580 * t2683;
    let t16534 = t121 * t5745;
    let t16687 = t2084 * t321;
    let t16692 = 1.0 / t2088 / t324;
    let t16705 = t1953 * t1959;
    (t16136, t16239, t16251, t16455, t16534, t16687, t16692, t16705)
}
