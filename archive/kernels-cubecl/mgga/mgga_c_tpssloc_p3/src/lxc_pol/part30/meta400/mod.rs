//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1524;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1525;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta400<F: Float>(t17691: F, t4588: F, t4582: F, t14187: F, t17686: F, t5878: F, t884: F, t3071: F, t1616: F, t4347: F, t376: F, t5866: F, t4594: F, t1023: F, t1041: F, t10413: F, t10436: F, t10511: F, t10871: F, t14049: F, t14059: F, t17688: F, t3039: F, t3070: F, t3114: F, t3130: F, t4585: F, t4590: F, t4644: F, t5869: F) -> (F, F, F, F, F, F, F) {
        let (t17693, t17697, t17701, t17705, t17712) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1524::<F>(t17691, t4588, t4582, t14187, t17686, t5878, t884, t3071, t1616, t4347, t376, t5866);
        let (t17714, t17718, t17725) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1525::<F>(t17712, t4594, t4582, t1023, t1041, t10413, t10436, t10511, t10871, t14049, t14059, t17688, t17693, t17697, t17701, t17705, t3039, t3070, t3114, t3130, t4585, t4590, t4644, t5869);
    (t17693, t17697, t17701, t17705, t17714, t17718, t17725)
}
