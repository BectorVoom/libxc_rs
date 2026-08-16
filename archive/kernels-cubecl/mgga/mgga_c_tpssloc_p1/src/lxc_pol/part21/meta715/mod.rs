//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta715 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2554;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta715<F: Float>(t13969: F, t13981: F, t3130: F, t10422: F, t14129: F, t3070: F, t11002: F, t14508: F, t10895: F, t14511: F, t14207: F, t3103: F, t14085: F, t3053: F, t14080: F, t10936: F, t4669: F, t14077: F, t1036: F, t14114: F, t3082: F, t4617: F, t10904: F, t14025: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t49940, t49945, t49957, t49959, t49964) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2554::<F>(t13969, t13981, t3130, t10422, t14129, t3070, t11002, t14508, t10895, t14511, t14207, t3103);
        let (t49966, t49972, t49984, t49987, t49989, t49993, t50027) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2555::<F>(t14085, t3053, t14080, t10936, t4669, t14077, t3103, t1036, t14114, t3082, t4617, t10904, t14025);
    (t49940, t49945, t49957, t49959, t49964, t49966, t49972, t49984, t49987, t49989, t49993, t50027)
}
