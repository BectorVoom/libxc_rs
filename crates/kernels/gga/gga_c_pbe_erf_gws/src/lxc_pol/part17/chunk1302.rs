//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1302/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1302<F: Float>(t3179: F, t51291: F, t854: F, t51244: F, t54075: F, t54077: F, t54080: F, t54082: F, t54085: F, t54088: F, t54092: F, t54094: F, t54096: F, t54098: F) -> F {
    let t54101 = t51291 * t3179;
    let t54102 = t854 * t54101;
    let t54103 = F::new(7.0) / F::new(72.0) * t54102;
    let t54104 = -t54075 / F::new(48.0) + t54077 / F::new(768.0) - t54080 / F::new(48.0) + t54082 / F::new(48.0) - t54085 / F::new(48.0) + t54088 - t54092 / F::new(12.0) + F::new(35.0) / F::new(432.0) * t54094 - t54096 / F::new(768.0) + t54098 / F::new(128.0) - F::new(7.0) / F::new(288.0) * t51244 + t54103;
    t54104
}
