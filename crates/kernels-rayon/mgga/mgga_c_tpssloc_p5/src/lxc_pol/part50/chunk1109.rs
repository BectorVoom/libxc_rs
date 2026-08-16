//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1109/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1109(t32984: f64, t33012: f64, t1484: f64, t1530: f64, t1877: f64, t193: f64, t202: f64, t2522: f64, t30757: f64, t30770: f64, t32885: f64, t6670: f64, t7540: f64, t8366: f64, t8370: f64, t870: f64) -> (f64, f64) {
    let t33013 = t32984 + t33012;
    let t33043 = t193 * t202 * t32885 * t870 + 3.0_f64 * t1484 * t2522 * t8366 - 3.0_f64 * t1484 * t2522 * t8370 - t1530 * t1877 * t30757 + 2.0_f64 * t1530 * t1877 * t30770 - 2.0_f64 * t1877 * t6670 * t7540;
    (t33013, t33043)
}
