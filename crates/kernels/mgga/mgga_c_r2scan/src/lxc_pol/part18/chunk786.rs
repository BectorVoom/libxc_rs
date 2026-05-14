//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 786/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk786<F: Float>(t8884: F, t8896: F, t8901: F, t8912: F, t8925: F, t8934: F, t8945: F, t8954: F, t8961: F, t8964: F, t8977: F, t8984: F, t9000: F, t9012: F, t9017: F, t9025: F) -> (F,) {
    let t9029 = t8884 + t8896 + t8901 + t8912 + t8925 + t8934 + t8945 + t8954 + t8961 + t8964 + t8977 + t8984 + t9000 + t9012 + t9017 + t9025;
    (t9029,)
}
