//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1413;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1414;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1415;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1416;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta392(t1089: f64, t16558: f64, t1088: f64, t123: f64, t11137: f64, t11459: f64, t14702: f64, t14720: f64, t14946: f64, t14947: f64, t18203: f64, t18208: f64, t18213: f64, t18217: f64, t18219: f64, t18223: f64, t18227: f64, t18229: f64, t18234: f64, t18239: f64, t423: f64, t14858: f64, t1703: f64, t4869: f64, t4879: f64, t1117: f64, t6021: f64, t3264: f64, t3315: f64, t6020: f64, t3313: f64, t4781: f64, t4785: f64, t11277: f64, t5988: f64, t11275: f64, t3411: f64, t6106: f64, t1157: f64, t6105: f64, t1164: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18241, t18243) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1413(t1089, t16558, t1088, t123);
        let t18245 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1414(t11137, t11459, t14702, t14720, t14946, t14947, t18203, t18208, t18213, t18217, t18219, t18223, t18227, t18229, t18234, t18239, t18243);
        let (t18247, t18249, t18251, t18257, t18261) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1415(t18245, t423, t14858, t1703, t4869, t4879, t1117, t6021, t3264, t3315, t6020, t3313);
        let (t18264, t18268, t18270, t18273) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1416(t4781, t4785, t3313, t11277, t5988, t1117, t11275, t3411, t6106, t1157, t6105, t1164);
    (t18241, t18243, t18247, t18249, t18251, t18257, t18261, t18264, t18268, t18270, t18273)
}
