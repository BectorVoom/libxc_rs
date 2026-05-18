//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1162/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1162<F: Float>(t16536: F, t16539: F, t16544: F, t16548: F, t16563: F, t16569: F, t16575: F, t16578: F, t19722: F, t19726: F, t19729: F, t19730: F, t19732: F, t19733: F, t19734: F, t19735: F) -> F {
    let t20320 = t16536 - t16539 - t16544 + t16548 + t19722 + t19726 - t19729 - t16563 - t19730 + t16569 - t19732 + t16575 + t16578 + t19733 + t19734 - t19735;
    t20320
}
