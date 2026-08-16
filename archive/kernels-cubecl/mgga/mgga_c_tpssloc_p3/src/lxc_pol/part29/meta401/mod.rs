//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1643;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1644;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1645;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta401<F: Float>(t15621: F, t4582: F, t11721: F, t3507: F, t4977: F, t3509: F, t1216: F, t15553: F, t13969: F, t4979: F, t3506: F, t4973: F, t1227: F, t11705: F, t11719: F, t11728: F, t11734: F, t11746: F, t15610: F, t15612: F, t15617: F, t3490: F, t3496: F, t3515: F, t4974: F, t4984: F, t5019: F, t12652: F, t4972: F, t11153: F, t3584: F, t14165: F, t1734: F, t3508: F, t1089: F, t1215: F, t607: F, t3578: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15622, t15625, t15627, t15631, t15637, t15640, t15642, t15643) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1643::<F>(t15621, t4582, t11721, t3507, t4977, t3509, t1216, t15553, t13969, t4979, t3506, t4973);
        let t15648 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1644::<F>(t1227, t15643, t11705, t11719, t11728, t11734, t11746, t15610, t15612, t15617, t15622, t15627, t15631, t15637, t15642, t3490, t3496, t3506, t3515, t4974, t4984, t5019);
        let (t15650, t15656, t15661, t15663) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1645::<F>(t12652, t4972, t4582, t11153, t3584, t14165, t1734, t3508, t1089, t1215, t607, t3578);
    (t15622, t15625, t15627, t15631, t15637, t15640, t15643, t15648, t15650, t15656, t15661, t15663)
}
