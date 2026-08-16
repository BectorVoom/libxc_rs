//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 658/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk658<F: Float>(t10876: F, t10878: F, t10881: F, t10885: F, t10888: F, t10891: F, t10899: F, t10901: F, t10905: F, t10908: F, t10911: F, t10918: F, t10921: F, t10923: F, t9852: F, t9891: F) -> F {
    let t12202 = F::cast_from(0.85206502119823888171e-1_f64) * t9852 - t10876 + t10878 + t10881 - t10885 + t10888 - t10891 + t10899 - F::cast_from(0.38342925953920749677e0_f64) * t9891 + t10901 + t10905 - t10908 + t10911 - t10918 + t10921 + t10923;
    t12202
}
