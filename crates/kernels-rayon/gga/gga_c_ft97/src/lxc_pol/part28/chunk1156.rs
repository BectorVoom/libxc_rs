//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1156/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1156(t148417: f64, t39693: f64, t446: f64, t32063: f64, t32888: f64, t34809: f64, t34918: f64, t558: f64, t1369: f64, t2112: f64, t28: f64, t139507: f64, t139519: f64, t139526: f64, t139534: f64, t148640: f64, t148643: f64, t148646: f64, t148649: f64, t148653: f64, t148657: f64, t148660: f64, t148667: f64, t148670: f64) -> (f64, f64, f64, f64, f64) {
    let t148673 = t446 * t39693 * t148417;
    let t148676 = t32888 * t32063 * t34809;
    let t148678 = t34918 * t558;
    let t148681 = t1369 * t28 * t2112 * t148678;
    let t148683 = t148640 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t148643 + 2.0_f64 / 9.0_f64 * t148646 - 2.0_f64 / 27.0_f64 * t148649 + 2.0_f64 / 9.0_f64 * t148653 - 2.0_f64 * t148657 + t148660 / 18.0_f64 - t139507 / 27.0_f64 + 2.0_f64 / 27.0_f64 * t139519 + t139526 / 18.0_f64 - t139534 + t148667 / 3.0_f64 + 4.0_f64 / 9.0_f64 * t148670 - 4.0_f64 / 27.0_f64 * t148673 - t148676 / 3.0_f64 + t148681 / 3.0_f64;
    (t148673, t148676, t148678, t148681, t148683)
}
