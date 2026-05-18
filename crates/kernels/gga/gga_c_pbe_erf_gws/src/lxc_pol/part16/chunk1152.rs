//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1152/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1152<F: Float>(t1206: F, t3189: F, t9283: F, t14605: F, t1115: F, t13810: F, t14188: F, t14327: F, t14479: F, t14585: F, t14589: F, t14593: F, t14597: F, t14603: F, t14883: F, t14888: F, t3066: F, t3207: F, t6793: F, t8793: F) -> (F, F, F) {
    let t14894 = t1206 * t3189;
    let t14895 = t9283 * t14894;
    let t14898 = F::new(7.0) / F::new(2304.0) * t14605;
    let t14899 = -t13810 + t14479 / F::new(48.0) - t1115 * t14327 / F::new(96.0) - t14585 / F::new(768.0) - t14589 / F::new(768.0) - t14593 / F::new(192.0) - t14597 / F::new(768.0) - t3066 * t14883 / F::new(16.0) + t6793 * t14888 / F::new(48.0) + t8793 * t14188 / F::new(48.0) + t14603 / F::new(384.0) - t3207 * t14895 / F::new(16.0) + t14898;
    (t14894, t14895, t14899)
}
