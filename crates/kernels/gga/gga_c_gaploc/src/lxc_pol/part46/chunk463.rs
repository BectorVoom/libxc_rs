//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 463/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk463<F: Float>(t123: F, t9065: F, t488: F, t2300: F, t2317: F, t6525: F, t122: F, t481: F, t880: F) -> (F, F, F, F) {
    let t9066 = t9065 * t123;
    let t9067 = t9066 * t488;
    let t9070 = t2300 * t2317;
    let t9072 = 0.23712505529730124666e-2 * t6525 * t9070;
    let t9074 = t481 * t880 * t122;
    (t9066, t9067, t9072, t9074)
}
