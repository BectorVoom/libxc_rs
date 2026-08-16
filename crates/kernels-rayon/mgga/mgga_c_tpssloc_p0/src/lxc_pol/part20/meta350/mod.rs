//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1656;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1657;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1658;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta350(t3732: f64, t792: f64, t118: f64, t3734: f64, t794: f64, t3719: f64, t3739: f64, t782: f64, t3736: f64, t1365: f64, t154: f64, t205: f64, t12156: f64, t210: f64, t214: f64, t1307: f64, t213: f64, t221: f64, t116: f64, t547: f64, t212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12204, t12205, t12208, t12209, t12211) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1656(t3732, t792, t118, t3734, t794, t3719, t3739, t782);
        let (t12212, t12214) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1657(t12211, t3736, t1365, t154);
        let (t12215, t12217, t12222, t12225, t12226) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1658(t12214, t205, t12156, t210, t214, t1307, t213, t221, t3719, t116, t547, t212);
    (t12204, t12205, t12208, t12209, t12211, t12212, t12214, t12215, t12217, t12222, t12225, t12226)
}
