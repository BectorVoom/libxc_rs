//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1398;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1399;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1400;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1401;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta470<F: Float>(t77119: F, t77122: F, t77124: F, t77127: F, t77130: F, t77133: F, t77135: F, t77138: F, t77143: F, t77145: F, t77148: F, t77150: F, t77153: F, t5946: F, t193: F, t3216: F, t336: F, t4700: F, t5950: F, t60874: F, t77157: F, t77159: F, t77224: F, t77226: F, t77229: F, t77232: F, t77236: F, t77470: F, t77474: F, t77478: F, t77482: F, t25: F, t265: F, t394: F, t76559: F, t76666: F, t77918: F, t1408: F, t1409: F, t1534: F, t1642: F, t20216: F, t20217: F, t21076: F, t21703: F, t396: F, t40: F, t5397: F, t5398: F, t5669: F, t5955: F, t75911: F, t75912: F, dens_threshold: F, rho0: F, zeta_threshold: F, t43791: F, t75836: F, t11219: F, t136: F, t43763: F, t43761: F, t3242: F, t75847: F, t3297: F, t3247: F, t1113: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t77920 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1398::<F>(t77119, t77122, t77124, t77127, t77130, t77133, t77135, t77138, t77143, t77145, t77148, t77150, t77153);
        let t77929 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1399::<F>(t5946, t193, t3216, t336, t4700, t5950, t60874, t77157, t77159, t77224, t77226, t77229, t77232, t77236, t77470, t77474, t77478, t77482);
        let t77944 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1400::<F>(t25, t265, t394, t76559, t76666, t77918, t77920, t77929, t1408, t1409, t1534, t1642, t20216, t20217, t21076, t21703, t396, t40, t5397, t5398, t5669, t5955, t75911, t75912, dens_threshold, rho0, zeta_threshold);
        let (t77953, t77957, t77959, t77961, t77963, t77965, t77967, t77969, t77971) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1401::<F>(t75911, t43791, t75836, t11219, t136, t43763, t43761, t3242, t75847, t3297, t3247, t1113);
    (t77944, t77953, t77957, t77959, t77961, t77963, t77965, t77967, t77969, t77971)
}
