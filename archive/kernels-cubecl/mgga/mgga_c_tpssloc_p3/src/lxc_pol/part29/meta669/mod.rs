//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta669 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2235;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2236;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2237;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2238;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2239;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta669<F: Float>(t22844: F, t6976: F, t22828: F, t7708: F, t16391: F, t26309: F, t5259: F, t80820: F, t16265: F, t22833: F, t5293: F, t80816: F, t80767: F, t80776: F, t80761: F, t80769: F, t91183: F, t91185: F, t91187: F, t91189: F, t91192: F, t91196: F, t91200: F, t91204: F, t91206: F, t22779: F, t26292: F, t1339: F, t54258: F, t550: F, t6936: F, t22827: F, t3788: F, t3792: F, t54068: F, t12289: F, t3791: F, t54014: F, t16311: F, t80784: F, t80792: F, t80794: F, t1825: F, t26288: F, t3734: F, t80780: F, t80789: F, t80796: F, t80801: F, t80807: F, t80814: F, t80821: F, t80826: F, t80828: F, t16314: F, t16227: F, t57643: F, t56805: F, t54165: F, t16060: F, t6944: F) -> (F, F, F, F, F, F, F, F) {
        let (t91210, t91212, t91215, t91216, t91218) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2235::<F>(t22844, t6976, t22828, t7708, t16391, t26309, t5259, t80820, t16265, t22833, t5293, t80816);
        let t91224 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2236::<F>(t80767, t80776, t80761, t80769, t91183, t91185, t91187, t91189, t91192, t91196, t91200, t91204, t91206, t91210, t91212, t91215, t91216, t91218);
        let (t91226, t91229, t91233, t91237) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2237::<F>(t22779, t26292, t1339, t54258, t550, t6936, t22827, t3788, t3792, t54068, t12289, t3791, t54014);
        let t91258 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2238::<F>(t16311, t3788, t3791, t6936, t80784, t80792, t80794, t1339, t1825, t26288, t3734, t80780, t80789, t80796, t80801, t80807, t80814, t80821, t80826, t80828, t91226, t91229, t91233, t91237);
        let (t91261, t91263, t91268, t91272, t91276, t91278) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2239::<F>(t16314, t26309, t16227, t22833, t1339, t57643, t6936, t22827, t550, t56805, t54165, t16060, t6944);
    (t91224, t91258, t91261, t91263, t91268, t91272, t91276, t91278)
}
