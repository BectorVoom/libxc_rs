//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 907/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk907(t112: f64, t34136: f64, t1458: f64, t2039: f64, t27863: f64, t32350: f64, t33152: f64, t33154: f64, t33583: f64, t33585: f64, t33587: f64, t33595: f64, t33598: f64, t33600: f64, t33690: f64, t7266: f64, t7801: f64, t8446: f64) -> (f64, f64) {
    let t34137 = t34136 * t112;
    let t34146 = 2.0_f64 * t1458 * t32350 + 2.0_f64 * t2039 * t27863 + 2.0_f64 * t2039 * t33690 + 2.0_f64 * t7266 * t7801 + t33152 + t33154 + t33583 + t33585 + t33587 + t33595 + t33598 + t33600 + t34137 + t8446;
    (t34137, t34146)
}
