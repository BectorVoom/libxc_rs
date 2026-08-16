//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta688 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2130;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2131;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta688<F: Float>(t1873: F, t96709: F, t5464: F, t81442: F, t666: F, t81446: F, t1453: F, t4067: F, t22473: F, t22470: F, t5488: F, t19529: F, t6530: F, t109: F, t81438: F, t81440: F, t86589: F, t86591: F, t92121: F, t1268: F, t28030: F, t6535: F, t26114: F, t7461: F, t19994: F, t24995: F, t8945: F, t28831: F, t83886: F, t6287: F, t652: F, t6534: F) -> (F, F, F, F, F, F, F, F) {
        let (t96711, t96713, t96716, t96719, t96721, t96724, t96726) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2130::<F>(t1873, t96709, t5464, t81442, t666, t81446, t1453, t4067, t22473, t22470, t5488, t19529, t6530);
        let (t96729, t96731) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2131::<F>(t109, t81438, t81440, t86589, t86591, t92121, t96713, t96716, t96719, t96721, t96724, t96726, t1268);
        let (t96738, t96740, t96746, t96755, t96758) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2132::<F>(t28030, t6535, t26114, t7461, t19994, t24995, t8945, t28831, t83886, t6287, t652, t6534);
    (t96711, t96729, t96731, t96738, t96740, t96746, t96755, t96758)
}
