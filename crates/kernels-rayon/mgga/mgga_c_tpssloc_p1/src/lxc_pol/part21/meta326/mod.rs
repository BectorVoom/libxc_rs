//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1695;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1696;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta326(t12199: f64, t3741: f64, t3732: f64, t792: f64, t118: f64, t3734: f64, t794: f64, t3719: f64, t3739: f64, t782: f64, t3736: f64, t1365: f64, t154: f64, t205: f64, t116: f64, t547: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12200, t12202, t12204, t12205, t12208, t12209, t12211) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1695(t12199, t3741, t3732, t792, t118, t3734, t794, t3719, t3739, t782);
        let (t12212, t12214, t12215, t12225) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1696(t12211, t3736, t1365, t154, t205, t116, t547);
    (t12200, t12202, t12204, t12205, t12208, t12209, t12211, t12212, t12214, t12215, t12225)
}
