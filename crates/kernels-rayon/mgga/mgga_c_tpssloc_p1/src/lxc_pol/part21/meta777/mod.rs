//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta777 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2687;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2688;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2689;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta777(t54432: f64, t54434: f64, t39596: f64, t39601: f64, t19644: f64, t225: f64, t20038: f64, t5353: f64, t12030: f64, t12444: f64, t1323: f64, t1372: f64, t1375: f64, t1385: f64, t1386: f64, t1843: f64, t19804: f64, t20009: f64, t20022: f64, t20023: f64, t20026: f64, t20029: f64, t3758: f64, t3882: f64, t3887: f64, t3912: f64, t53866: f64, t54825: f64, t55069: f64, t55150: f64, t568: f64, t6440: f64, t6461: f64, t212: f64, t6330: f64, t2586: f64, t40353: f64, t6347: f64, t12225: f64, t40343: f64, t40347: f64, t40350: f64, t40351: f64, t40356: f64, t40360: f64, t54631: f64, t54633: f64, t54635: f64, t54637: f64, t54639: f64, t54643: f64, t118: f64, t19631: f64, t3739: f64, t794: f64, t40018: f64, t6353: f64, t5187: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56411, t56412, t56416, t56417, t56457) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2687(t54432, t54434, t39596, t39601, t19644, t225, t20038, t5353, t12030, t12444, t1323, t1372, t1375, t1385, t1386, t1843, t19804, t20009, t20022, t20023, t20026, t20029, t3758, t3882, t3887, t3912, t53866, t54825, t55069, t55150, t568, t6440, t6461);
        let (t56463, t56467, t56475) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2688(t212, t6330, t2586, t40353, t6347, t12225, t40343, t40347, t40350, t40351, t40356, t40360, t54631, t54633, t54635, t54637, t54639, t54643);
        let (t56482, t56484, t56486) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2689(t118, t19631, t3739, t794, t40018, t6353, t5187);
    (t56411, t56412, t56416, t56417, t56457, t56463, t56467, t56475, t56482, t56484, t56486)
}
