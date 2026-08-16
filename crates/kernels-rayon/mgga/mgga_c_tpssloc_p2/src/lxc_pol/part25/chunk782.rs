//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 782/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk782(t2745: f64, t870: f64, t2553: f64, t262: f64, t2752: f64, t1877: f64, t2522: f64, t4314: f64, t776: f64, t868: f64, t9684: f64, t9715: f64, t9718: f64, t9724: f64, t9727: f64, t9780: f64, t9789: f64, t9863: f64, t9865: f64, t9867: f64, t9870: f64) -> f64 {
    let t10126 = t2745 * t870;
    let t10130 = t262 * t2553;
    let t10134 = t2745 * t2752;
    let t10138 = 9.0_f64 * t10126 * t2522 * t776 + 18.0_f64 * t10130 * t4314 * t776 - 3.0_f64 * t10134 * t1877 * t868 + t9684 - t9715 - t9718 + t9724 + t9727 + t9780 - t9789 + t9863 + t9865 - t9867 + t9870;
    t10138
}
