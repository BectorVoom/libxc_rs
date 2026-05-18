//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1055/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1055<F: Float>(t1960: F, t5511: F, t157: F, t1814: F, t406: F, t8397: F, t8423: F, t33795: F, t615: F, t1658: F, t2146: F, t2147: F, t2331: F, t29992: F, t30006: F, t30011: F, t30015: F, t33444: F, t33451: F, t33459: F, t33465: F, t33468: F, t7912: F, t7932: F, t8400: F, t8403: F, t9794: F) -> (F, F, F) {
    let t38644 = t1960 * t5511;
    let t38647 = t1814 * t406 * t157;
    let t38660 = t8397 * t8423;
    let t38662 = t615 * t33795;
    let t38665 = F::new(0.17347256376410398924e1) * t33444 - F::new(0.8673628188205199462e0) * t29992 + F::new(0.65854491829355115987e0) * t38644 + F::new(0.4336814094102599731e0) * t8400 * t7932 * t38647 - F::new(0.34694512752820797848e1) * t33451 + t33459 + F::new(0.17347256376410398924e1) * t33465 + F::new(0.17347256376410398924e1) * t2146 * t2147 * t2331 * t1658 + F::new(0.8673628188205199462e0) * t7912 * t9794 + t33468 + F::new(0.17347256376410398924e1) * t30006 - F::new(0.17347256376410398924e1) * t38660 + t30011 + F::new(0.8673628188205199462e0) * t38662 * t8403 + t30015;
    (t38647, t38662, t38665)
}
