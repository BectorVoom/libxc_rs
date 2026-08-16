//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta396 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1510;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1511;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1512;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1513;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta396<F: Float>(t17488: F, t291: F, t2932: F, t5790: F, t950: F, t4471: F, t4475: F, t10632: F, t5774: F, t13727: F, t4359: F, t13520: F, t4400: F, t5695: F, t912: F, t2842: F, t1557: F, t4395: F, t2792: F, t5730: F, t10661: F, t10756: F, t10828: F, t17192: F, t17451: F, t17454: F, t17471: F, t2905: F, t2930: F, t311: F, t5727: F, t2844: F, t5726: F, t4399: F, t10704: F, t5694: F, t10702: F, t5743: F, t931: F, t1569: F, t4433: F, t5762: F, t5759: F, t2888: F, t5758: F, t4437: F, t10813: F, t5742: F, t10771: F, t10811: F, t14271: F, t14276: F, t2861: F, t2886: F, t4416: F, t4438: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17490, t17493, t17496, t17500, t17504, t17506) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1510::<F>(t17488, t291, t2932, t5790, t950, t4471, t4475, t10632, t5774, t13727, t4359, t13520, t4400);
        let (t17509, t17512, t17515, t17516) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1511::<F>(t5695, t912, t2842, t1557, t4395, t2792, t5730, t10661, t10756, t10828, t17192, t17451, t17454, t17471, t17490, t17493, t17496, t17500, t17504, t17506, t2905, t2930, t311);
        let (t17519, t17523, t17526, t17530, t17535) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1512::<F>(t5727, t912, t2792, t2844, t5726, t2842, t4395, t4399, t10704, t5694, t10702, t5743, t931);
        let t17558 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1513::<F>(t1569, t4433, t5762, t931, t5759, t2888, t5758, t4437, t10813, t5742, t10771, t10811, t14271, t14276, t17519, t17523, t17526, t17530, t17535, t2861, t2886, t4416, t4438);
    (t17490, t17504, t17506, t17509, t17512, t17515, t17516, t17519, t17523, t17526, t17530, t17558)
}
