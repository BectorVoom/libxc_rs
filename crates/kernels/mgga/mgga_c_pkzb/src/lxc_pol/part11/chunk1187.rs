//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1187/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1187<F: Float>(t16536: F, t16539: F, t16544: F, t16548: F, t16563: F, t16569: F, t16575: F, t16578: F, t28942: F, t28950: F, t28951: F, t28952: F, t28954: F, t28955: F, t28956: F, t28957: F) -> F {
    let t29114 = t16536 - t16539 - t16544 + t16548 - t28942 - t28950 - t16563 + t16569 - t28951 + t16575 + t16578 + t28952 + t28954 - t28955 - t28956 - t28957;
    t29114
}
