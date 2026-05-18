//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 490/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk490<F: Float>(t1022: F, t2101: F, t2300: F, t2317: F, t6525: F, t122: F, t481: F, t880: F) -> (F, F, F) {
    let t9014 = t2101 * t1022;
    let t9070 = t2300 * t2317;
    let t9072 = F::new(0.23712505529730124666e-2) * t6525 * t9070;
    let t9074 = t481 * t880 * t122;
    (t9014, t9072, t9074)
}
