//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1849;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1850;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta466<F: Float>(t20594: F, t225: F, t554: F, t12215: F, t1341: F, t1363: F, t16285: F, t1827: F, t19855: F, t19940: F, t19942: F, t20512: F, t20516: F, t20556: F, t20565: F, t20570: F, t3733: F, t5235: F, t559: F, t6390: F, t6422: F, t16211: F, t1831: F, t19834: F, t19839: F, t19841: F, t19851: F, t19904: F, t20433: F, t20442: F, t20484: F, t20508: F, t3803: F, t5240: F, t6427: F, t6431: F, t539: F, t1842: F, t6439: F, t12021: F, t6460: F, t3887: F, t553: F, t12249: F, t20490: F, t20495: F, t3897: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20595, t20596, t20599) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1849::<F>(t20594, t225, t554, t12215, t1341, t1363, t16285, t1827, t19855, t19940, t19942, t20512, t20516, t20556, t20565, t20570, t3733, t5235, t559, t6390, t6422);
        let t20601 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1850::<F>(t1363, t16211, t1831, t19834, t19839, t19841, t19851, t19904, t20433, t20442, t20484, t20508, t20599, t3803, t5240, t6427, t6431);
        let (t20602, t20608, t20609, t20613, t20616, t20622, t20625) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1851::<F>(t20601, t539, t1842, t6439, t12021, t6460, t3887, t553, t12249, t20490, t20495, t3897);
    (t20595, t20596, t20601, t20602, t20608, t20609, t20613, t20616, t20622, t20625)
}
