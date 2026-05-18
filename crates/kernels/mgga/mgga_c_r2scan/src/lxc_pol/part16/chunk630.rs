//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 630/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk630<F: Float>(t1053: F, t1102: F, t3457: F, t3267: F, t3273: F, t3280: F, t3351: F, t3355: F, t3432: F, t3442: F, t3445: F, t3451: F, t3455: F) -> F {
    let t3459 = t1102 * t1053 * t3457;
    let t3461 = -t3432 + t3442 - t3445 - t3451 - F::new(0.36021158228745895953e-3) * t3455 + F::new(0.15243824895787514157e-3) * t3459 - t3267 - t3273 + t3280 - t3351 + t3355;
    t3461
}
