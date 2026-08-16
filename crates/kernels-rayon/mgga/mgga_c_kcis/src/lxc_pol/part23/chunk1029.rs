//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1029/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1029(t26651: f64, t783: f64, t26392: f64, t26400: f64, t26410: f64, t26520: f64, t26556: f64, t26558: f64, t26561: f64, t26633: f64, t26634: f64, t2771: f64, t7660: f64, t899: f64, t9010: f64) -> (f64, f64) {
    let t26652 = t783 * t26651;
    let t26653 = -t26556 * t899 + 4.0_f64 * t26561 * t2771 + 2.0_f64 * t26634 * t2771 + 4.0_f64 * t7660 * t9010 + t26392 - t26400 + t26410 + t26520 + t26558 - t26633 + t26652;
    (t26652, t26653)
}
