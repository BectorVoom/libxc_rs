//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1661/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1661(t13487: f64, t1877: f64, t193: f64, t202: f64, t2057: f64, t2379: f64, t24334: f64, t24339: f64, t24344: f64, t2522: f64, t2553: f64, t2745: f64, t2749: f64, t4314: f64, t7110: f64, t7114: f64, t776: f64, t868: f64, t870: f64) -> f64 {
    let t24379 = t193 * t202 * t24334 * t870 - 6.0_f64 * t13487 * t2522 * t7114 - 2.0_f64 * t1877 * t24339 * t868 + 2.0_f64 * t1877 * t24344 * t2749 - t1877 * t2745 * t7114 + 6.0_f64 * t2057 * t2379 * t4314 + 3.0_f64 * t2057 * t2522 * t2553 + 6.0_f64 * t2522 * t7110 * t776;
    t24379
}
