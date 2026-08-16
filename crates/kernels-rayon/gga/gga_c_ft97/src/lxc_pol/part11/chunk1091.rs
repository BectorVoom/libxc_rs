//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1091/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1091(t42724: f64, t42740: f64, t42757: f64, t42772: f64, t762: f64, t10153: f64, t10157: f64, t1901: f64, t242: f64, t2579: f64, t258: f64, t2599: f64, t2606: f64, t3892: f64, t42332: f64, t42404: f64, t42690: f64, t42697: f64, t42703: f64, t42708: f64, t446: f64, t684: f64, t729: f64, t773: f64, t9692: f64, t9708: f64) -> (f64, f64) {
    let t42775 = t762 * (t42724 + t42740 + t42757 + t42772);
    let t42783 = 4.0_f64 * t446 * t729 * t10153 * t2579 - 4.0_f64 * t1901 * t2606 * t3892 * t42404 - 4.0_f64 / 9.0_f64 * t42690 + 4.0_f64 / 9.0_f64 * t1901 * t2599 * t258 * t9692 * t684 + 16.0_f64 / 9.0_f64 * t42697 - 4.0_f64 / 3.0_f64 * t446 * t729 * t773 * t9692 - 16.0_f64 / 9.0_f64 * t42703 + 2.0_f64 * t446 * t242 * t42332 + 4.0_f64 / 3.0_f64 * t42708 - t446 * t242 * t42775 / 3.0_f64 - 8.0_f64 * t446 * t10157 * t773 * t9708;
    (t42775, t42783)
}
