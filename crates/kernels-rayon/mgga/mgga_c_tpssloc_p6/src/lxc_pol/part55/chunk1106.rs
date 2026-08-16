//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1106/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1106(t25: f64, t7540: f64, t1408: f64, t1877: f64, t2522: f64, t30757: f64, t30770: f64, t32886: f64, t6670: f64, t7475: f64, t7545: f64, t8366: f64, t8370: f64) -> (f64, f64) {
    let t32899 = t25 * t7540;
    let t32907 = 3.0_f64 / 2.0_f64 * t2522 * t8366 * t7475 + t1877 * t32886 * t25 / 2.0_f64 - t1877 * t30757 * t7545 / 2.0_f64 + t1877 * t8366 * t1408 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t2522 * t8370 * t7475 - t1877 * t6670 * t32899 + t1877 * t30770 * t7545 - t1877 * t8370 * t1408 / 2.0_f64;
    (t32899, t32907)
}
