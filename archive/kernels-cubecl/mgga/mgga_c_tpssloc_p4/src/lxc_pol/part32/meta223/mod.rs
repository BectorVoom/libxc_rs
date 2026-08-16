//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta223 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1035;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1036;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1037;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta223<F: Float>(t3188: F, t5928: F, t1615: F, t1625: F, t1060: F, t381: F, t5866: F, t3201: F, t383: F, t5914: F, t1058: F, t1610: F, t1630: F, t1632: F, t3186: F, t3200: F, t353: F, t384: F, t4669: F, t5903: F, t1055: F, t1052: F, t1635: F, t388: F, t4557: F, t4660: F, t5849: F, t5851: F, t5915: F, t5920: F, t1637: F, t1070: F, t193: F, t3216: F, t336: F, t5691: F, t5693: F, t5697: F, t5729: F, t5732: F, t5798: F, t5800: F, t5802: F, t5806: F, t5810: F, t5814: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5929, t5932, t5933, t5936, t5937, t5939, t5941, t5943) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1035::<F>(t3188, t5928, t1615, t1625, t1060, t381, t5866, t3201, t383, t5914, t1058, t1610, t1630, t1632, t3186, t3200, t353, t384, t4669, t5903);
        let (t5944, t5946, t5950) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1036::<F>(t1055, t5943, t1052, t1635, t388, t4557, t4660, t5849, t5851, t5915, t5920, t1637);
        let t5954 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1037::<F>(t1070, t193, t3216, t336, t5691, t5693, t5697, t5729, t5732, t5798, t5800, t5802, t5806, t5810, t5814, t5946, t5950);
    (t5929, t5932, t5933, t5936, t5937, t5939, t5941, t5943, t5944, t5946, t5950, t5954)
}
