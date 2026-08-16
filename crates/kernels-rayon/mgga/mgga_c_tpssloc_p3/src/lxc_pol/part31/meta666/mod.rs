//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta666 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1955;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1956;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1957;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1958;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1959;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta666(t1081: f64, t5527: f64, t16596: f64, t89992: f64, t23788: f64, t98007: f64, t17109: f64, t28: f64, t25365: f64, t98058: f64, t25927: f64, t98003: f64, t1395: f64, t5456: f64, t2105: f64, t6470: f64, t1851: f64, t7961: f64, t1404: f64, t1858: f64, t20149: f64, t20186: f64, t2099: f64, t27241: f64, t29396: f64, t5364: f64, t5381: f64, t6483: f64, t7223: f64, t7946: f64, t91830: f64, t91832: f64, t91834: f64, t91842: f64, t109: f64, t84036: f64, t86583: f64, t86586: f64, t92122: f64, t92123: f64, t96713: f64, t96716: f64, t96719: f64, t96721: f64, t96724: f64, t96726: f64, t2098: f64, t671: f64, t112: f64, t29395: f64, t12524: f64, t1401: f64, t1458: f64, t16524: f64, t19534: f64, t20176: f64, t24462: f64, t24465: f64, t27170: f64, t27273: f64, t27276: f64, t28951: f64, t29422: f64, t29425: f64, t33185: f64, t3938: f64, t5371: f64, t5376: f64, t5493: f64, t55388: f64, t7230: f64, t7235: f64, t75795: f64, t7956: f64, t94127: f64, t94170: f64, t19289: f64, t19451: f64, t1983: f64, t2039: f64, t2095: f64, t2314: f64, t24987: f64, t24995: f64, t26114: f64, t26161: f64, t26179: f64, t26558: f64, t26875: f64, t27150: f64, t27171: f64, t27219: f64, t27226: f64, t29197: f64, t29211: f64, t35259: f64, t4028: f64, t4034: f64, t4072: f64, t5308: f64, t57806: f64, t6468: f64, t652: f64, t7057: f64, t7166: f64, t7458: f64, t7802: f64, t7890: f64, t7941: f64, t96830: f64, t97890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t100759, t100766, t100769, t100772, t100780, t100788, t100791) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1955(t1081, t5527, t16596, t89992, t23788, t98007, t17109, t28, t25365, t98058, t25927, t98003);
        let (t100930, t100976) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1956(t1395, t5456, t2105, t6470, t1851, t7961, t1404, t1858, t20149, t20186, t2099, t27241, t29396, t5364, t5381, t6483, t7223, t7946, t91830, t91832, t91834, t91842);
        let (t100990, t100993) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1957(t109, t84036, t86583, t86586, t92122, t92123, t96713, t96716, t96719, t96721, t96724, t96726, t2098, t671);
        let t101021 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1958(t112, t29395, t100990, t100993, t12524, t1401, t1458, t16524, t19534, t20176, t24462, t24465, t27170, t27273, t27276, t28951, t29422, t29425, t33185, t3938, t5371, t5376, t5456, t5493, t55388, t671, t7230, t7235, t75795, t7956, t94127, t94170);
        let t101091 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1959(t19289, t19451, t1983, t2039, t2095, t2314, t24987, t24995, t26114, t26161, t26179, t26558, t26875, t27150, t27171, t27219, t27226, t29197, t29211, t35259, t4028, t4034, t4072, t5308, t57806, t6468, t652, t671, t7057, t7166, t7458, t7802, t7890, t7941, t96830, t97890);
    (t100759, t100766, t100769, t100772, t100780, t100788, t100791, t100930, t100976, t100990, t101021, t101091)
}
