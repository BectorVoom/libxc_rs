//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1828;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1829;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta485(t483: f64, t3068: f64, t1244: f64, sigma2: f64, t2132: f64, t24683: f64, t225: f64, t460: f64, t479: f64, t23413: f64, t3523: f64, t7345: f64, t3572: f64, t7339: f64, t1218: f64, t1232: f64, t2134: f64, t2136: f64, t24704: f64, t24706: f64, t24712: f64, t24716: f64, t24723: f64, t24729: f64, t24733: f64, t24736: f64, t3496: f64, t3511: f64, t3518: f64, t3527: f64, t3531: f64, t3580: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24739, t24740, t24741) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1828(t483, t3068, t1244, sigma2);
        let (t24744, t24745, t24746) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1829(t2132, t24683, t225, t460, t479);
        let (t24747, t24749, t24752, t24754, t24756) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1830(t24744, t24746, t2132, t23413, t3523, t7345, t3572, t7339, t1218, t1232, t2134, t2136, t24704, t24706, t24712, t24716, t24723, t24729, t24733, t24736, t24741, t3496, t3511, t3518, t3527, t3531, t3580);
    (t24739, t24740, t24741, t24745, t24746, t24747, t24749, t24752, t24754, t24756)
}
