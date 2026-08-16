//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta652 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2274;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2275;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2276;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2277;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2278;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2279;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta652(t1268: f64, t86604: f64, t1873: f64, t55934: f64, t12725: f64, t6534: f64, t55962: f64, t19456: f64, t4072: f64, t649: f64, t26114: f64, t12813: f64, t88: f64, t22479: f64, t4028: f64, t1458: f64, t2363: f64, t24999: f64, t83935: f64, t90351: f64, t90352: f64, t90355: f64, t2311: f64, t7676: f64, t7467: f64, t9348: f64, t45632: f64, t111: f64, t26097: f64, t12734: f64, t2314: f64, t26135: f64, t12739: f64, t5113: f64, t22461: f64, t26103: f64, t6517: f64, t671: f64, t90041: f64, t90044: f64, t1983: f64, t23857: f64, t7753: f64, t24991: f64, t6876: f64, t12728: f64, t1459: f64, t16503: f64, t1976: f64, t1980: f64, t23829: f64, t24980: f64, t4034: f64, t4037: f64, t574: f64, t652: f64, t90034: f64, t90036: f64, t90038: f64, t90040: f64, t90051: f64, t90059: f64, t90062: f64, t90064: f64, t90068: f64, t25992: f64, t22592: f64, t7685: f64, t22948: f64, t5161: f64, t1845: f64, t3914: f64, t26161: f64, t26162: f64, t24994: f64, t6875: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90361, t90363, t90365, t90367, t90369, t90370, t90372, t90374, t90375) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2274(t1268, t86604, t1873, t55934, t12725, t6534, t55962, t19456, t4072, t649, t26114, t12813, t88);
        let t90380 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2275(t1873, t90375, t22479, t4028, t1458, t2363, t24999, t83935, t90351, t90352, t90355, t90361, t90363, t90365, t90367, t90369, t90372, t90374);
        let (t90381, t90383, t90385, t90387, t90399, t90400, t90404, t90406) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2276(t1458, t2311, t1873, t22479, t7676, t7467, t9348, t45632, t111, t26097, t12734, t2314, t26135);
        let t90411 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2277(t12739, t7467, t26135, t5113, t12813, t1458, t22461, t26103, t4072, t6517, t671, t90041, t90044, t90383, t90385, t90387, t90399, t90400, t90404, t90406);
        let t90422 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2278(t1983, t23857, t7753, t24991, t6876, t12728, t1458, t1459, t16503, t1976, t1980, t23829, t24980, t26103, t4034, t4037, t574, t652, t90034, t90036, t90038, t90040, t90041, t90044, t90051, t90059, t90062, t90064, t90068, t90380, t90411);
        let (t90428, t90434, t90436, t90440, t90442) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2279(t25992, t6876, t22592, t7685, t1983, t22948, t5161, t1845, t3914, t26161, t26162, t24994, t6875);
    (t90370, t90381, t90400, t90422, t90428, t90434, t90436, t90440, t90442)
}
