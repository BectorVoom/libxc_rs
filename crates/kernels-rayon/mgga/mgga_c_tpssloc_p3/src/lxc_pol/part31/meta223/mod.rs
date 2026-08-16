//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta223 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk958;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk959;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk960;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta223(t3188: f64, t5928: f64, t1615: f64, t1625: f64, t1060: f64, t381: f64, t5866: f64, t3201: f64, t383: f64, t5914: f64, t1058: f64, t1610: f64, t1630: f64, t1632: f64, t3186: f64, t3200: f64, t353: f64, t384: f64, t4669: f64, t5903: f64, t1055: f64, t1052: f64, t1635: f64, t388: f64, t4557: f64, t4660: f64, t5849: f64, t5851: f64, t5915: f64, t5920: f64, t1637: f64, t1070: f64, t193: f64, t3216: f64, t336: f64, t5691: f64, t5693: f64, t5697: f64, t5729: f64, t5732: f64, t5798: f64, t5800: f64, t5802: f64, t5806: f64, t5810: f64, t5814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5929, t5932, t5933, t5936, t5937, t5939, t5941, t5943) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk958(t3188, t5928, t1615, t1625, t1060, t381, t5866, t3201, t383, t5914, t1058, t1610, t1630, t1632, t3186, t3200, t353, t384, t4669, t5903);
        let (t5944, t5946, t5950) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk959(t1055, t5943, t1052, t1635, t388, t4557, t4660, t5849, t5851, t5915, t5920, t1637);
        let t5954 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk960(t1070, t193, t3216, t336, t5691, t5693, t5697, t5729, t5732, t5798, t5800, t5802, t5806, t5810, t5814, t5946, t5950);
    (t5929, t5932, t5933, t5936, t5937, t5939, t5941, t5943, t5944, t5946, t5950, t5954)
}
