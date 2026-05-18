//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 910/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk910<F: Float>(t10167: F, t10176: F, t10132: F, t10134: F, t10144: F, t10147: F, t10151: F, t138: F, t1572: F, t1577: F, t2902: F, t2905: F, t2919: F, t3675: F, t3683: F, t514: F, t520: F, t5847: F, t5854: F, t8206: F, t8209: F, t985: F) -> F {
    let t10177 = t10167 + t10176;
    let t10179 = t10132 * t138 - t10134 * t520 - F::new(6.0) * t10144 * t5854 + F::new(4.0) * t10147 * t1577 + F::new(2.0) * t10151 * t1577 - t10177 * t514 - t1572 * t3683 - F::new(2.0) * t2902 * t2919 + F::new(4.0) * t2905 * t8209 + F::new(2.0) * t3675 * t5847 - F::new(2.0) * t8206 * t985;
    t10179
}
