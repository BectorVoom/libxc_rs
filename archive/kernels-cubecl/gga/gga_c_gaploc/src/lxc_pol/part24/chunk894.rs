//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 894/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk894<F: Float>(t8911: F, t8957: F, t8988: F, t9030: F, t1052: F, t2208: F, t2300: F, t2317: F, t6525: F, t122: F, t481: F, t880: F) -> (F, F, F, F, F) {
    let t9032 = t8911 + t8957 + t8988 + t9030;
    let t9034 = t1052 * t2208;
    let t9070 = t2300 * t2317;
    let t9072 = F::cast_from(0.23712505529730124666e-2_f64) * t6525 * t9070;
    let t9074 = t481 * t880 * t122;
    (t9032, t9034, t9070, t9072, t9074)
}
