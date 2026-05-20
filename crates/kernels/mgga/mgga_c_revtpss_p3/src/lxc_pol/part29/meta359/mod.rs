//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1292;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1293;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta359<F: Float>(t12916: F, t3722: F, t3718: F, t3172: F, t3590: F, t1247: F, t3612: F, t3610: F, t1260: F, t3666: F, t3713: F, t3711: F, t127: F, t3661: F, t371: F, t1235: F, t12640: F, t225: F, t12657: F, t480: F, t3667: F, t3678: F, t1236: F, t676: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12918, t12942, t12949, t12956, t12960) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1292::<F>(t12916, t3722, t3718, t3172, t3590, t1247, t3612, t3610, t1260, t3666, t3713, t3711);
        let (t12964, t12966, t12975, t12976, t12979, t12984) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1293::<F>(t127, t3661, t371, t1235, t12640, t225, t12657, t480, t3667, t3678, t1236, t676);
    (t12918, t12942, t12949, t12956, t12960, t12964, t12966, t12975, t12976, t12979, t12984)
}
