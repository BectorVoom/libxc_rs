//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta240 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1001;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1002;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1003;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1004;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1005;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1006;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1007;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta240<F: Float>(t1799: F, t550: F, t3805: F, t5249: F, t5264: F, t5266: F, t2408: F, t2417: F, t2426: F, t2486: F, t3688: F, t3813: F, t6299: F, t6304: F, t6329: F, t2423: F, t3686: F, t3690: F, t3695: F, t3819: F, t3821: F, t3823: F, t3825: F, t3832: F, t3836: F, t6300: F, t6322: F, t225: F, t3843: F, t6330: F, t1347: F, t6347: F, t1819: F, t1821: F, t546: F, t548: F, t1343: F, t820: F, t6387: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6394, t6396) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1001::<F>(t1799, t550, t3805, t5249);
        let (t6399, t6400, t6401, t6402) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1002::<F>(t5264, t5266, t2408, t2417, t2426, t2486, t3688, t3813, t6299, t6304, t6329, t2423, t3686, t3690, t3695, t3819, t3821, t3823, t3825, t3832, t3836, t6300, t6322);
        let (t6404, t6408, t6411, t6414) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1003::<F>(t225, t6401, t6402, t3843, t6330, t1347, t6347, t1819, t1821, t546, t548);
        let t6415 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1004::<F>(t550, t6414);
        let t6417 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1005::<F>(t1343, t6415, t820);
        let t6420 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1006::<F>(t550, t6387);
        let t6422 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1007::<F>(t1343, t6420, t820);
    (t6394, t6396, t6399, t6400, t6404, t6408, t6411, t6414, t6415, t6417, t6420, t6422)
}
