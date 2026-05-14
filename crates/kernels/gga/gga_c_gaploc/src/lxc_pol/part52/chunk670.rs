//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 670/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk670<F: Float>(t16692: F, t201: F, t2536: F, t2925: F, t1022: F, t7275: F, t32356: F, t739: F, t10938: F, t2021: F, t10007: F, t10627: F, t1890: F, t10600: F, t1415: F, t31585: F, t493: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33332 = t201 * t16692;
    let t33348 = t2536 * t2925;
    let t33360 = t7275 * t1022;
    let t33561 = t739 * t32356;
    let t33565 = t2021 * t10938;
    let t33601 = t10007 * t10627;
    let t33760 = t1890 * t32356;
    let t34264 = t1415 * t10600;
    let t34267 = t493 * t31585;
    (t33332, t33348, t33360, t33561, t33565, t33601, t33760, t34264, t34267)
}
