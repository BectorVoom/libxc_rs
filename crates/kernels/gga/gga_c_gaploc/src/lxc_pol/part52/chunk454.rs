//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 454/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk454<F: Float>(t161: F, t8773: F, t1023: F, t1853: F, t1022: F, t2101: F, t2300: F, t2317: F, t6525: F, t122: F, t481: F, t880: F) -> (F, F, F, F, F) {
    let t8878 = t8773 * t161;
    let t8942 = t1023 * t1853;
    let t9014 = t2101 * t1022;
    let t9070 = t2300 * t2317;
    let t9072 = 0.23712505529730124666e-2 * t6525 * t9070;
    let t9074 = t481 * t880 * t122;
    (t8878, t8942, t9014, t9072, t9074)
}
