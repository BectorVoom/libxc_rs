//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 815/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk815(t1413: f64, t1449: f64, t2481: f64, t2507: f64, t3311: f64, t3337: f64, t4218: f64, t430: f64, t453: f64, t459: f64, t4772: f64, t4828: f64, t6634: f64, t8599: f64, t8604: f64, t8607: f64, t8610: f64, t8615: f64, t8661: f64, t8664: f64, t8667: f64, t8670: f64, t8673: f64, t8676: f64, t8705: f64) -> f64 {
    let t8708 = 0.33125e-1_f64 * t4218 * t2507 - 0.33125e-1_f64 * t8599 * t459 - 0.6625e-1_f64 * t6634 * t3311 + 0.19875e0_f64 * t4772 * t8604 - 0.6625e-1_f64 * t1413 * t8607 - 0.6625e-1_f64 * t1413 * t8610 + 0.165625e-1_f64 * t2481 * t3337 - 0.33125e-1_f64 * t1413 * t8615 + 0.165625e-1_f64 * t430 * t8661 + 0.99375e-1_f64 * t4772 * t8664 - 0.19875e0_f64 * t4828 * t8667 + 0.99375e-1_f64 * t1449 * t8670 - 0.33125e-1_f64 * t1413 * t8673 + 0.496875e-1_f64 * t1449 * t8676 - 0.165625e-1_f64 * t453 * t8705;
    t8708
}
