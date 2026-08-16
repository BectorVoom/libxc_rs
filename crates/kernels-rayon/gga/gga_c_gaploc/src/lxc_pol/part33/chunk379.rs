//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 379/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk379(t238: f64, t622: f64, t233: f64, t629: f64, t630: f64, t1112: f64, t1114: f64, t1116: f64, t1144: f64, t1146: f64, t1148: f64, t241: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1776 = t622 * t238;
    let t1777 = 1.0_f64 / t1776;
    let t1778 = t233 * t1777;
    let t1779 = t629 * t629;
    let t1780 = t1779 * t630;
    let t1789 = -0.78438333333333333333e0_f64 * t1112 + 0.15687666666666666667e1_f64 * t1114 + 0.68863333333333333333e0_f64 * t1116 + 0.14025833333333333333e0_f64 * t1144 + 0.28051666666666666667e0_f64 * t1146 + 0.17365833333333333333e0_f64 * t1148;
    let t1790 = t1789 * t630;
    let t1793 = t622 * t622;
    let t1794 = 1.0_f64 / t1793;
    let t1795 = t233 * t1794;
    let t1796 = t241 * t241;
    (t1778, t1779, t1780, t1790, t1795, t1796)
}
