//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta180 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk940;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk941;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk942;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk943;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk944;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk945;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk946;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk947;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta180(t1388: f64, t1390: f64, t1297: f64, t1307: f64, t193: f64, t2408: f64, t2417: f64, t3683: f64, t3686: f64, t3688: f64, t3690: f64, t3693: f64, t3695: f64, t3697: f64, t3698: f64, t3701: f64, t3719: f64, t3813: f64, t3914: f64, t3918: f64, t533: f64, t531: f64, t571: f64, t2423: f64, t2426: f64, t2486: f64, t3734: f64, t3816: f64, t3819: f64, t3821: f64, t3823: f64, t3825: f64, t3828: f64, t3830: f64, t3832: f64, t3834: f64, t3836: f64, t113: f64, t1266: f64, t1271: f64, t1393: f64, t2312: f64, t2314: f64, t2320: f64, t2323: f64, t2364: f64, t3652: f64, t3660: f64, t510: f64, t513: f64, t574: f64, t650: f64, t652: f64, t672: f64, t3: f64, t112: f64, t1395: f64, t111: f64, t576: f64, t1401: f64, t2319: f64, t2363: f64, t577: f64, t671: f64, t2218: f64, t2221: f64, t2225: f64, t2232: f64, t1406: f64, t604: f64, t1437: f64, t645: f64, t1409: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3919, t3923) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk940(t1388, t1390, t1297, t1307, t193, t2408, t2417, t3683, t3686, t3688, t3690, t3693, t3695, t3697, t3698, t3701, t3719, t3813, t3914, t3918, t533);
        let t3928 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk941(t531, t571, t193, t2423, t2426, t2486, t3734, t3816, t3819, t3821, t3823, t3825, t3828, t3830, t3832, t3834, t3836);
        let (t3929, t3931) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk942(t3923, t3928, t113, t1266, t1271, t1393, t2312, t2314, t2320, t2323, t2364, t3652, t3660, t510, t513, t574, t650, t652, t672);
        let (t3932, t3938) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk943(t3, t3931, t112, t1395);
        let t3941 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk944(t111, t576);
        let (t3946, t3951) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk945(t1401, t2319, t2363, t3931, t3938, t3941, t577, t671, t2218, t2221, t2225, t2232);
        let t3953 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk946(t1406, t604);
        let (t3958, t3961) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk947(t1437, t645, t1409, t607);
    (t3919, t3929, t3931, t3932, t3938, t3941, t3946, t3951, t3953, t3958, t3961)
}
