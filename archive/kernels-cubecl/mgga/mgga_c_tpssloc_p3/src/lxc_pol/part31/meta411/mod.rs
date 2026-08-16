//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1506;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1507;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1508;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1509;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1510;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta411<F: Float>(t16132: F, t1825: F, t1352: F, t19743: F, t19660: F, t118: F, t6330: F, t794: F, t12202: F, t19631: F, t210: F, t214: F, t6347: F, t3739: F, t12211: F, t6353: F, t213: F, t1307: F, t221: F, t5187: F, t5196: F, t12188: F, t12190: F, t12194: F, t12196: F, t12200: F, t1315: F, t16101: F, t5195: F, t3726: F, t6358: F, t12228: F, t12236: F, t16078: F, t16083: F, t16099: F, t16106: F, t16108: F, t16113: F, t16119: F, t225: F, t1814: F, t5343: F, t3901: F, t6420: F, t6378: F, t68: F) -> (F, F, F, F, F, F, F, F) {
        let (t19756, t19761, t19763, t19768, t19771) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1506::<F>(t16132, t1825, t1352, t19743, t19660, t118, t6330, t794, t12202, t19631, t210, t214);
        let (t19776, t19779, t19783, t19787) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1507::<F>(t118, t6347, t794, t3739, t12211, t6353, t213, t6330, t1307, t221, t5187, t5196);
        let t19790 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1508::<F>(t12188, t12190, t12194, t12196, t12200, t1315, t16101, t19768, t19771, t19776, t19779, t19783, t19787, t5195);
        let t19803 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1509::<F>(t3726, t6358, t213, t6347, t1307, t221, t12228, t12236, t16078, t16083, t16099, t16106, t16108, t16113, t16119, t5195);
        let (t19804, t19805, t19810, t19813, t19815) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1510::<F>(t19790, t19803, t225, t1814, t5343, t3901, t6420, t6378, t68);
    (t19756, t19761, t19763, t19804, t19805, t19810, t19813, t19815)
}
