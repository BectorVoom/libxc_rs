//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 680/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk680<F: Float>(t3005: F, t3295: F, t9800: F, t11053: F, t9805: F, t1029: F, t9796: F, t3247: F, t900: F, t10867: F, t10924: F, t787: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13052 = t3005 * t3295;
    let t13053 = t9800 * t13052;
    let t13055 = t11053 * t3295;
    let t13056 = t9805 * t13055;
    let t13058 = t1029 * t3295;
    let t13059 = t9796 * t13058;
    let t13072 = t900 * t3247;
    let t13073 = t10867 * t13072;
    let t13077 = t787 * t10924;
    (t13052, t13053, t13055, t13056, t13058, t13059, t13072, t13073, t13077)
}
