//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta428 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1845;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1846;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta428<F: Float>(t14480: F, t959: F, t2952: F, t4483: F, t10623: F, t1589: F, t14257: F, t14262: F, t14376: F, t14378: F, t14381: F, t14384: F, t14387: F, t14391: F, t14394: F, t14398: F, t14424: F, t14472: F, t14475: F, t14477: F, t14479: F, t14238: F, t360: F, t1021: F, t248: F, t3053: F, t4644: F, t10422: F, t4578: F, t3070: F, t1603: F, t3030: F, t3032: F, t3129: F, t3038: F, t1020: F, t10937: F, t10962: F, t10982: F, t10985: F, t10994: F, t11003: F, t14235: F, t1618: F, t3043: F, t3057: F, t3064: F, t3114: F, t3123: F, t3134: F, t4579: F, t4641: F, t4652: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14482, t14484, t14486, t14487) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1845::<F>(t14480, t959, t2952, t4483, t10623, t1589, t14257, t14262, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14424, t14472, t14475, t14477, t14479);
        let (t14488, t14489, t14491, t14495, t14501, t14503, t14506) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1846::<F>(t14238, t14487, t360, t1021, t248, t3053, t4644, t10422, t4578, t3070, t1603, t3030);
        let (t14507, t14508, t14511, t14523) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1847::<F>(t14506, t3032, t3129, t3038, t1020, t10937, t10962, t10982, t10985, t10994, t11003, t14235, t14491, t14495, t14503, t1618, t3043, t3057, t3064, t3070, t3114, t3123, t3134, t4579, t4641, t4644, t4652);
    (t14482, t14484, t14486, t14488, t14489, t14491, t14501, t14506, t14507, t14508, t14511, t14523)
}
