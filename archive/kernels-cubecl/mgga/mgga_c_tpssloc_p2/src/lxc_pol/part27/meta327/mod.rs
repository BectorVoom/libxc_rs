//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1405;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1406;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta327<F: Float>(t11820: F, t1213: F, t1226: F, t3566: F, t11552: F, t221: F, t456: F, t1197: F, t698: F, t1174: F, t135: F, t3551: F, t3556: F, t3493: F, t3612: F, t11812: F, t1243: F, t10471: F, t11715: F, t11712: F, t11721: F, t6739: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11821, t11825, t11834, t11836, t11838) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1405::<F>(t11820, t1213, t1226, t3566, t11552, t221, t456, t1197, t698, t1174, t135, t3551);
        let (t11839, t11842, t11871, t11877, t11881, t11883) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1406::<F>(t1174, t11838, t135, t3556, t3493, t3612, t11812, t1243, t10471, t11715, t11712, t11721, t6739);
    (t11821, t11825, t11834, t11836, t11839, t11842, t11871, t11877, t11881, t11883)
}
