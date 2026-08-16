//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1236/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1236(t108451: f64, t870: f64, t105732: f64, t105741: f64, t105745: f64, t105770: f64, t105773: f64, t105787: f64, t105801: f64, t105810: f64, t1408: f64, t1877: f64, t20216: f64, t2057: f64, t2058: f64, t24191: f64, t25: f64, t2522: f64, t26563: f64, t26756: f64, t28241: f64, t28249: f64, t29106: f64, t4314: f64, t5397: f64, t7114: f64, t7475: f64, t7845: f64, t92319: f64) -> (f64, f64) {
    let t108452 = t108451 * t870;
    let t108466 = 9.0_f64 * t4314 * t2057 * t105810 + 3.0_f64 / 2.0_f64 * t1877 * t29106 * t1408 + 9.0_f64 / 2.0_f64 * t2522 * t29106 * t7475 + 3.0_f64 * t26756 * t105770 + 9.0_f64 / 2.0_f64 * t2522 * t2057 * t105741 + 9.0_f64 / 2.0_f64 * t2522 * t2057 * t105745 - t1877 * t7114 * t105787 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t1877 * t7845 * t5397 + 3.0_f64 * t105773 * t2058 + 9.0_f64 * t26563 * t105801 + t1877 * t108452 * t25 / 2.0_f64 + 9.0_f64 * t24191 * t105732 + 9.0_f64 * t4314 * t7845 * t28241 - 9.0_f64 * t92319 * t28249 + t1877 * t2057 * t20216 / 2.0_f64;
    (t108452, t108466)
}
