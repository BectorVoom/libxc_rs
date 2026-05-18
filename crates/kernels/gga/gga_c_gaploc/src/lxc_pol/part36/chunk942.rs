//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 942/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk942<F: Float>(t11083: F, t2558: F, t943: F, t1897: F, t28957: F, t2936: F, t10782: F, t2580: F, t7068: F, t32112: F, t954: F, t13225: F, t731: F) -> (F, F, F, F, F) {
    let t43127 = t943 * t11083 * t2558;
    let t43131 = F::new(0.23071578690426672851e-1) * t1897 * t2936 * t28957;
    let t43134 = t1897 * t2580 * t10782 * t7068;
    let t43137 = t1897 * t954 * t32112;
    let t43139 = t731 * t13225;
    (t43127, t43131, t43134, t43137, t43139)
}
