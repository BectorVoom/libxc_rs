//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta384 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1358;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1359;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1360;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1361;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta384(t17488: f64, t291: f64, t2932: f64, t5790: f64, t950: f64, t4471: f64, t4475: f64, t10632: f64, t5774: f64, t13727: f64, t4359: f64, t13520: f64, t4400: f64, t5695: f64, t912: f64, t2842: f64, t1557: f64, t4395: f64, t2792: f64, t5730: f64, t10661: f64, t10756: f64, t10828: f64, t17192: f64, t17451: f64, t17454: f64, t17471: f64, t2905: f64, t2930: f64, t311: f64, t5727: f64, t2844: f64, t5726: f64, t4399: f64, t10704: f64, t5694: f64, t10702: f64, t5743: f64, t931: f64, t1569: f64, t4433: f64, t5762: f64, t5759: f64, t2888: f64, t5758: f64, t4437: f64, t10813: f64, t5742: f64, t10771: f64, t10811: f64, t14271: f64, t14276: f64, t2861: f64, t2886: f64, t4416: f64, t4438: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17490, t17493, t17496, t17500, t17504, t17506) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1358(t17488, t291, t2932, t5790, t950, t4471, t4475, t10632, t5774, t13727, t4359, t13520, t4400);
        let (t17509, t17512, t17515, t17516) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1359(t5695, t912, t2842, t1557, t4395, t2792, t5730, t10661, t10756, t10828, t17192, t17451, t17454, t17471, t17490, t17493, t17496, t17500, t17504, t17506, t2905, t2930, t311);
        let (t17519, t17523, t17526, t17530, t17535) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1360(t5727, t912, t2792, t2844, t5726, t2842, t4395, t4399, t10704, t5694, t10702, t5743, t931);
        let t17558 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1361(t1569, t4433, t5762, t931, t5759, t2888, t5758, t4437, t10813, t5742, t10771, t10811, t14271, t14276, t17519, t17523, t17526, t17530, t17535, t2861, t2886, t4416, t4438);
    (t17490, t17504, t17506, t17509, t17512, t17515, t17516, t17519, t17523, t17526, t17530, t17558)
}
