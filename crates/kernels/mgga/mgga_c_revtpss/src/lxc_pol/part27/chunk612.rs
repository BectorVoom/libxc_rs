//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 612/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk612<F: Float>(t3655: F, t481: F, t1231: F, t1256: F, t1247: F, t1261: F, t1266: F, t3591: F, t3600: F, t3606: F, t3610: F, t3613: F, t3620: F, t3625: F, t3631: F, t3637: F, t3640: F, t3644: F, t3647: F, t3651: F, t484: F) -> F {
    let t3657 = F::new(0.47637797908966374413e-4) * t481 * t3655;
    let t3658 = t1231 * t1256;
    let t3660 = F::new(0.21437009059034868486e-3) * t1247 * t3591 + F::new(0.42874018118069736972e-3) * t3600 * t3606 - F::new(0.21437009059034868486e-3) * t3610 * t3613 + F::new(0.23818898954483187207e-3) * t1261 * t3620 - F::new(0.28582678745379824648e-3) * t3625 * t3631 - F::new(0.19055119163586549765e-3) * t3637 - F::new(0.14291339372689912324e-3) * t1261 * t3640 - F::new(0.28582678745379824648e-3) * t1261 * t3644 - F::new(0.28582678745379824648e-3) * t3647 * t1266 + F::new(0.21437009059034868486e-3) * t3651 * t484 - t3657 + F::new(0.28582678745379824648e-3) * t3658;
    t3660
}
