//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 856/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk856(t242: f64, t35729: f64, t1175: f64, t729: f64, t7484: f64, t1449: f64, t6940: f64, t2568: f64, t35694: f64, t35699: f64, t35703: f64, t35707: f64, t35710: f64, t35714: f64, t35717: f64, t35721: f64, t35726: f64, t446: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35730 = t242 * t35729;
    let t35734 = t729 * t1175 * t7484;
    let t35737 = t1449 * t6940;
    let t35738 = t2568 * t35737;
    let t35739 = t242 * t35738;
    let t35742 = -t446 * t35694 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t35699 - 2.0_f64 / 3.0_f64 * t446 * t35703 - t446 * t35707 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t35710 - 2.0_f64 / 3.0_f64 * t446 * t35714 - 2.0_f64 / 3.0_f64 * t446 * t35717 + 2.0_f64 / 3.0_f64 * t446 * t35721 + 2.0_f64 / 3.0_f64 * t446 * t35726 + 2.0_f64 / 3.0_f64 * t446 * t35730 - t446 * t35734 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t35739;
    (t35730, t35734, t35737, t35738, t35739, t35742)
}
