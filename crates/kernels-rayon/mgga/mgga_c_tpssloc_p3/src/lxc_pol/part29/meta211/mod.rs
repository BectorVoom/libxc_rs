//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta211 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1031;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1032;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1033;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1034;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1035;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1036;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1037;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta211(t193: f64, t336: f64, t1637: f64, t3216: f64, t1068: f64, t1070: f64, t4353: f64, t4356: f64, t4358: f64, t4361: f64, t4398: f64, t4402: f64, t4480: f64, t4482: f64, t4485: f64, t4487: f64, t4491: f64, t4495: f64, t4500: f64, t4696: f64, t25: f64, t265: f64, t394: f64, t4324: f64, t1074: f64, t1408: f64, t1409: f64, t1534: f64, t1642: f64, t396: f64, t3966: f64, t40: f64, t4332: f64, t606: f64, t607: f64, t873: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1654: f64, t690: f64, t3242: f64, t3240: f64, t123: f64, t3247: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4700 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1031(t193, t336);
        let (t4701, t4704) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1032(t1637, t3216, t1068, t1070, t193, t336, t4353, t4356, t4358, t4361, t4398, t4402, t4480, t4482, t4485, t4487, t4491, t4495, t4500, t4696, t4700);
        let (t4705, t4712) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1033(t25, t265, t394, t4324, t4704, t1074, t1408, t1409, t1534, t1642, t396, t3966, t40, t4332, t606, t607, t873, dens_threshold, rho0, zeta_threshold);
        let t4721 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1034(t1654, t690);
        let t4723 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1035(t1409, t3242);
        let t4724 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1036(t4723, t607);
        let (t4725, t4726, t4728, t4729) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1037(t3240, t4724, t123, t1409, t3247, t607);
    (t4700, t4701, t4705, t4712, t4721, t4723, t4724, t4725, t4726, t4728, t4729)
}
