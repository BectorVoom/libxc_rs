//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta777 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2687;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2688;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2689;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta777<F: Float>(t54432: F, t54434: F, t39596: F, t39601: F, t19644: F, t225: F, t20038: F, t5353: F, t12030: F, t12444: F, t1323: F, t1372: F, t1375: F, t1385: F, t1386: F, t1843: F, t19804: F, t20009: F, t20022: F, t20023: F, t20026: F, t20029: F, t3758: F, t3882: F, t3887: F, t3912: F, t53866: F, t54825: F, t55069: F, t55150: F, t568: F, t6440: F, t6461: F, t212: F, t6330: F, t2586: F, t40353: F, t6347: F, t12225: F, t40343: F, t40347: F, t40350: F, t40351: F, t40356: F, t40360: F, t54631: F, t54633: F, t54635: F, t54637: F, t54639: F, t54643: F, t118: F, t19631: F, t3739: F, t794: F, t40018: F, t6353: F, t5187: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t56411, t56412, t56416, t56417, t56457) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2687::<F>(t54432, t54434, t39596, t39601, t19644, t225, t20038, t5353, t12030, t12444, t1323, t1372, t1375, t1385, t1386, t1843, t19804, t20009, t20022, t20023, t20026, t20029, t3758, t3882, t3887, t3912, t53866, t54825, t55069, t55150, t568, t6440, t6461);
        let (t56463, t56467, t56475) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2688::<F>(t212, t6330, t2586, t40353, t6347, t12225, t40343, t40347, t40350, t40351, t40356, t40360, t54631, t54633, t54635, t54637, t54639, t54643);
        let (t56482, t56484, t56486) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2689::<F>(t118, t19631, t3739, t794, t40018, t6353, t5187);
    (t56411, t56412, t56416, t56417, t56457, t56463, t56467, t56475, t56482, t56484, t56486)
}
