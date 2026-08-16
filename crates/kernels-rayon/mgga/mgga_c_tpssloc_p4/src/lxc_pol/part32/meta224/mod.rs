//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta224 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1038;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1039;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1040;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1041;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta224(t25: f64, t265: f64, t394: f64, t5669: f64, t5954: f64, t1408: f64, t1409: f64, t1534: f64, t1642: f64, t396: f64, t40: f64, t5397: f64, t5398: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t3242: f64, t5392: f64, t3240: f64, t123: f64, t3247: f64, t1088: f64, t1089: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5955, t5962) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1038(t25, t265, t394, t5669, t5954, t1408, t1409, t1534, t1642, t396, t40, t5397, t5398, dens_threshold, rho0, zeta_threshold);
        let t5966 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1039(t5397);
        let t5971 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1040(t3242, t5392);
        let (t5972, t5973, t5975) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1041(t3240, t5971, t123, t3247, t5392);
        let (t5976, t5977, t5979) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1042(t1088, t5975, t123, t1089, t5398);
    (t5955, t5962, t5966, t5971, t5972, t5973, t5975, t5976, t5977, t5979)
}
