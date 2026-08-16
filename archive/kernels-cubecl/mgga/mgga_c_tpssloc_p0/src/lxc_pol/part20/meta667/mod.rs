//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta667 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2509;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2510;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2511;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2512;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta667<F: Float>(t14783: F, t699: F, t14786: F, t14789: F, t50946: F, t50948: F, t50950: F, t50952: F, t50954: F, t50957: F, t50961: F, t50966: F, t136: F, t43761: F, t50924: F, t14778: F, t11219: F, t50910: F, t50915: F, t11153: F, t1229: F, t45971: F, t47774: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43816: F, t43895: F, t3242: F, t486: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t50968, t50970, t50972, t50974) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2509::<F>(t14783, t699, t14786, t14789, t50946, t50948, t50950, t50952, t50954, t50957, t50961, t50966);
        let (t50976, t50978, t50987, t50990, t50992, t50994) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2510::<F>(t136, t43761, t50924, t14778, t699, t11219, t50910, t50915, t11153, t1229, t45971, t47774);
        let t50996 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2511::<F>(t43780, t43782, t43784, t43786, t43788, t43816, t43895, t50976, t50978, t50987, t50990, t50994);
        let t51000 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2512::<F>(t3242, t486, t45971, t47774);
    (t50968, t50970, t50972, t50974, t50976, t50978, t50987, t50990, t50992, t50994, t50996, t51000)
}
