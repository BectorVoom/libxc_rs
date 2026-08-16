//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta686 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2599;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2600;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2601;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta686(t1734: f64, t3507: f64, t11721: f64, t3493: f64, t4978: f64, t11786: f64, t5005: f64, t15730: f64, t3536: f64, t15594: f64, t3523: f64, t11678: f64, t11684: f64, t11805: f64, t11809: f64, t1215: f64, t15569: f64, t15659: f64, t15660: f64, t15761: f64, t1653: f64, t2244: f64, t2250: f64, t3247: f64, t3490: f64, t3578: f64, t45197: f64, t5024: f64, t52687: f64, t1174: f64, t14726: f64, t44562: f64, t3577: f64, t44951: f64, t4953: f64, t11677: f64, t15245: f64, t11665: f64, t11668: f64, t11670: f64, t11694: f64, t1177: f64, t11853: f64, t1227: f64, t1230: f64, t15714: f64, t248: f64, t3243: f64, t3515: f64, t44851: f64, t44871: f64, t4582: f64, t4977: f64, t5012: f64, t50830: f64, t50929: f64, t14753: f64, t3431: f64, t14744: f64, t11651: f64, t15438: f64, t13969: f64, t15540: f64, t15530: f64, t11638: f64, t11688: f64, t15740: f64, t3506: f64, t3508: f64, t44621: f64, t44886: f64, t44890: f64, t44894: f64, t50924: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t52696, t52704, t52709, t52737) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2599(t1734, t3507, t11721, t3493, t4978, t11786, t5005, t15730, t3536, t15594, t3523, t11678, t11684, t11805, t11809, t1215, t15569, t15659, t15660, t15761, t1653, t2244, t2250, t3247, t3490, t3578, t45197, t5024, t52687);
        let t52769 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2600(t1174, t14726, t44562, t3577, t44951, t4953, t11677, t15245, t11665, t11668, t11670, t11694, t1177, t11853, t1227, t1230, t15569, t15714, t248, t3243, t3515, t44851, t44871, t4582, t4977, t5012, t50830, t50929);
        let t52797 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2601(t1174, t14753, t3431, t14744, t11651, t15438, t1227, t13969, t15540, t15530, t3515, t11638, t11688, t15740, t3506, t3508, t44621, t44886, t44890, t44894, t4582, t4977, t50924);
    (t52696, t52704, t52709, t52737, t52769, t52797)
}
