//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2816/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2816(t10143: f64, t5660: f64, t12895: f64, t1877: f64, t193: f64, t202: f64, t2522: f64, t2749: f64, t39585: f64, t39590: f64, t39593: f64, t4119: f64, t58139: f64, t58973: f64, t58974: f64, t58975: f64, t58978: f64, t58979: f64, t58980: f64, t59434: f64, t59475: f64, t59518: f64, t59558: f64, t766: f64, t870: f64) -> f64 {
    let t59564 = t5660 * t10143;
    let t59571 = 3.0_f64 * t193 * t766 * t58139 + t193 * t202 * (t59434 + t59475 + t59518 + t59558) * t870 + t58973 - t39585 + t39590 + t58974 - t39593 + t58975 + t58978 + 2.0_f64 * t1877 * t59564 * t2749 + t58979 - t58980 + 12.0_f64 * t2522 * t12895 * t4119;
    t59571
}
