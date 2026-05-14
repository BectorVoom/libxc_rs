//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1323/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1323<F: Float>(t31066: F, t569: F, t1453: F, t8273: F, t508: F, t2178: F, t4151: F, t10416: F, t1312: F, t13435: F, t13440: F, t18163: F, t2179: F, t2181: F, t2322: F, t31013: F, t31016: F, t4254: F, t5523: F, t651: F, t8254: F, t8274: F, t8278: F, t8280: F) -> (F, F, F, F, F) {
    let t31067 = t31066 * t569;
    let t31070 = t8273 * t1453;
    let t31073 = t508 * t31066;
    let t31084 = t2178 * t4151;
    let t31087 = -2.0 * t10416 * t2179 + 2.0 * t10416 * t2181 + 2.0 * t1312 * t31067 + 4.0 * t1312 * t31070 + 2.0 * t1312 * t31084 - 4.0 * t13435 * t2179 + 4.0 * t13435 * t2181 + 2.0 * t13440 * t2181 - 2.0 * t18163 * t2179 - 4.0 * t2322 * t8254 - 4.0 * t2322 * t8274 + 4.0 * t2322 * t8278 + 4.0 * t2322 * t8280 - 2.0 * t31013 * t651 - 4.0 * t31016 * t651 - 2.0 * t31073 * t651 - 4.0 * t4254 * t8254 - 4.0 * t4254 * t8274 + 4.0 * t5523 * t8278 + 4.0 * t5523 * t8280;
    (t31067, t31070, t31073, t31084, t31087)
}
