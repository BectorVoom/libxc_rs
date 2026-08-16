//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1666;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1667;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1668;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1669;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1670;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta433(t16132: f64, t1825: f64, t1352: f64, t19743: f64, t19660: f64, t118: f64, t6330: f64, t794: f64, t12202: f64, t19631: f64, t210: f64, t214: f64, t6347: f64, t3739: f64, t12211: f64, t6353: f64, t213: f64, t1307: f64, t221: f64, t5187: f64, t5196: f64, t12188: f64, t12190: f64, t12194: f64, t12196: f64, t12200: f64, t1315: f64, t16101: f64, t5195: f64, t3726: f64, t6358: f64, t12228: f64, t12236: f64, t16078: f64, t16083: f64, t16099: f64, t16106: f64, t16108: f64, t16113: f64, t16119: f64, t225: f64, t1814: f64, t5343: f64, t3901: f64, t6420: f64, t6378: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19756, t19761, t19763, t19768, t19771) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1666(t16132, t1825, t1352, t19743, t19660, t118, t6330, t794, t12202, t19631, t210, t214);
        let (t19776, t19779, t19783, t19787) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1667(t118, t6347, t794, t3739, t12211, t6353, t213, t6330, t1307, t221, t5187, t5196);
        let t19790 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1668(t12188, t12190, t12194, t12196, t12200, t1315, t16101, t19768, t19771, t19776, t19779, t19783, t19787, t5195);
        let t19803 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1669(t3726, t6358, t213, t6347, t1307, t221, t12228, t12236, t16078, t16083, t16099, t16106, t16108, t16113, t16119, t5195);
        let (t19804, t19805, t19810, t19813, t19815) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1670(t19790, t19803, t225, t1814, t5343, t3901, t6420, t6378, t68);
    (t19756, t19761, t19763, t19804, t19805, t19810, t19813, t19815)
}
