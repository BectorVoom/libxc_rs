//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 883/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk883<F: Float>(t502: F, t50820: F, t50832: F, t50841: F, t50843: F, t50849: F, t50858: F, t50871: F, t50880: F, t50884: F, t50887: F, t50891: F, t50893: F, t50902: F, t50911: F, t50917: F, t50925: F) -> (F,) {
    let t50930 = t502 * (t50820 + t50832 + t50841 + t50843 + t50849 + t50858 + t50871 + t50880 + t50884 + t50887 + t50891 + t50893 + t50902 + t50911 + t50917 + t50925);
    (t50930,)
}
