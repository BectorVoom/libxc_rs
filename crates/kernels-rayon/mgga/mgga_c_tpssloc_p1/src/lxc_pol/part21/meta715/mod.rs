//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta715 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2554;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta715(t13969: f64, t13981: f64, t3130: f64, t10422: f64, t14129: f64, t3070: f64, t11002: f64, t14508: f64, t10895: f64, t14511: f64, t14207: f64, t3103: f64, t14085: f64, t3053: f64, t14080: f64, t10936: f64, t4669: f64, t14077: f64, t1036: f64, t14114: f64, t3082: f64, t4617: f64, t10904: f64, t14025: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49940, t49945, t49957, t49959, t49964) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2554(t13969, t13981, t3130, t10422, t14129, t3070, t11002, t14508, t10895, t14511, t14207, t3103);
        let (t49966, t49972, t49984, t49987, t49989, t49993, t50027) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2555(t14085, t3053, t14080, t10936, t4669, t14077, t3103, t1036, t14114, t3082, t4617, t10904, t14025);
    (t49940, t49945, t49957, t49959, t49964, t49966, t49972, t49984, t49987, t49989, t49993, t50027)
}
