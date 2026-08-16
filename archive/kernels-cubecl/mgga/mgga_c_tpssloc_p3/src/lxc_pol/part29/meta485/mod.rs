//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1828;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1829;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta485<F: Float>(t483: F, t3068: F, t1244: F, sigma2: F, t2132: F, t24683: F, t225: F, t460: F, t479: F, t23413: F, t3523: F, t7345: F, t3572: F, t7339: F, t1218: F, t1232: F, t2134: F, t2136: F, t24704: F, t24706: F, t24712: F, t24716: F, t24723: F, t24729: F, t24733: F, t24736: F, t3496: F, t3511: F, t3518: F, t3527: F, t3531: F, t3580: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t24739, t24740, t24741) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1828::<F>(t483, t3068, t1244, sigma2);
        let (t24744, t24745, t24746) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1829::<F>(t2132, t24683, t225, t460, t479);
        let (t24747, t24749, t24752, t24754, t24756) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1830::<F>(t24744, t24746, t2132, t23413, t3523, t7345, t3572, t7339, t1218, t1232, t2134, t2136, t24704, t24706, t24712, t24716, t24723, t24729, t24733, t24736, t24741, t3496, t3511, t3518, t3527, t3531, t3580);
    (t24739, t24740, t24741, t24745, t24746, t24747, t24749, t24752, t24754, t24756)
}
