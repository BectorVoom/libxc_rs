//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 893/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk893(t1458: f64, t2039: f64, t24999: f64, t31532: f64, t33085: f64, t33152: f64, t33154: f64, t33579: f64, t33583: f64, t33585: f64, t33587: f64, t33595: f64, t33598: f64, t33600: f64, t6517: f64, t7801: f64, t8446: f64) -> f64 {
    let t33601 = 2.0_f64 * t1458 * t31532 + 2.0_f64 * t2039 * t24999 + 2.0_f64 * t2039 * t33085 + 2.0_f64 * t6517 * t7801 + t33152 + t33154 + t33579 + t33583 + t33585 + t33587 + t33595 + t33598 + t33600 + t8446;
    t33601
}
