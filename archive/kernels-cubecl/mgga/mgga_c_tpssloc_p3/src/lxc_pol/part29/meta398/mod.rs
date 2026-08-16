//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1633;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1634;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1635;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta398<F: Float>(t15437: F, t3514: F, t3572: F, t5002: F, t3523: F, t5005: F, t5019: F, t5024: F, t11147: F, t11778: F, t14165: F, t4582: F, t1735: F, t3252: F, t3578: F, t3248: F, t11642: F, t11644: F, t11649: F, t1174: F, t1227: F, t15434: F, t3518: F, t3527: F, t3531: F, t3577: F, t1216: F, t4733: F, t1653: F, t3494: F, t1090: F, t5012: F, t3490: F, t4993: F, t248: F, t3521: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15438, t15446, t15448, t15450, t15452, t15455) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1633::<F>(t15437, t3514, t3572, t5002, t3523, t5005, t5019, t5024, t11147, t11778, t14165, t4582);
        let (t15459, t15463, t15466) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1634::<F>(t1735, t3252, t3578, t3248, t11642, t11644, t11649, t1174, t1227, t15434, t15438, t15446, t15448, t15450, t15452, t15455, t3518, t3527, t3531, t3577, t5005);
        let (t15470, t15474, t15478, t15484, t15486, t15488) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1635::<F>(t1216, t4733, t3578, t1653, t3494, t1090, t5012, t3490, t4993, t248, t3521, t1227);
    (t15455, t15459, t15463, t15466, t15470, t15474, t15478, t15484, t15486, t15488)
}
