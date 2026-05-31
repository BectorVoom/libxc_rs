//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 806/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk806<F: Float>(t169: F, t5750: F, t1683: F, t5335: F, t5344: F, t92: F, t291: F, t293: F, t5343: F, t5747: F, t801: F, t6059: F, t769: F) -> (F, F, F, F, F, F) {
    let t15499 = t169 * t5750;
    let t15665 = F::cast_from(1.0_f64) / t5335 / t1683;
    let t15667 = t15665 * t92 * t5344;
    let t15672 = t291 / t5343 / t293;
    let t15751 = t801 * t5747;
    let t15766 = t769 * t6059;
    (t15499, t15665, t15667, t15672, t15751, t15766)
}
