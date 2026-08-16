//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta664 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2330;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2331;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2332;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2333;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2334;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta664(t22844: f64, t6976: f64, t22828: f64, t7708: f64, t16391: f64, t26309: f64, t5259: f64, t80820: f64, t16265: f64, t22833: f64, t5293: f64, t80816: f64, t80767: f64, t80776: f64, t80761: f64, t80769: f64, t91183: f64, t91185: f64, t91187: f64, t91189: f64, t91192: f64, t91196: f64, t91200: f64, t91204: f64, t91206: f64, t22779: f64, t26292: f64, t1339: f64, t54258: f64, t550: f64, t6936: f64, t22827: f64, t3788: f64, t3792: f64, t54068: f64, t12289: f64, t3791: f64, t54014: f64, t16311: f64, t80784: f64, t80792: f64, t80794: f64, t1825: f64, t26288: f64, t3734: f64, t80780: f64, t80789: f64, t80796: f64, t80801: f64, t80807: f64, t80814: f64, t80821: f64, t80826: f64, t80828: f64, t16314: f64, t16227: f64, t57643: f64, t56805: f64, t54165: f64, t16060: f64, t6944: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91210, t91212, t91215, t91216, t91218) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2330(t22844, t6976, t22828, t7708, t16391, t26309, t5259, t80820, t16265, t22833, t5293, t80816);
        let t91224 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2331(t80767, t80776, t80761, t80769, t91183, t91185, t91187, t91189, t91192, t91196, t91200, t91204, t91206, t91210, t91212, t91215, t91216, t91218);
        let (t91226, t91229, t91233, t91237) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2332(t22779, t26292, t1339, t54258, t550, t6936, t22827, t3788, t3792, t54068, t12289, t3791, t54014);
        let t91258 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2333(t16311, t3788, t3791, t6936, t80784, t80792, t80794, t1339, t1825, t26288, t3734, t80780, t80789, t80796, t80801, t80807, t80814, t80821, t80826, t80828, t91226, t91229, t91233, t91237);
        let (t91261, t91263, t91268, t91272, t91276, t91278) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2334(t16314, t26309, t16227, t22833, t1339, t57643, t6936, t22827, t550, t56805, t54165, t16060, t6944);
    (t91224, t91258, t91261, t91263, t91268, t91272, t91276, t91278)
}
