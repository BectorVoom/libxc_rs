//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 799/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk799(t10508: f64, t10672: f64, t10734: f64, t10806: f64, t10660: f64, t312: f64, t10236: f64, t10238: f64, t10432: f64, t10667: f64, t10680: f64, t10689: f64, t10699: f64, t10713: f64, t10800: f64, t2649: f64, t2745: f64, t2892: f64, t301: f64, t317: f64, t830: f64, t880: f64) -> (f64, f64, f64) {
    let t10808 = t10508 + t10672 + t10734 + t10806;
    let t10810 = t10660 * t312;
    let t10818 = -t10236 * t317 - 2.0_f64 * t10238 * t317 - t10432 * t317 - t10808 * t301 - 3.0_f64 * t2649 * t880 - 3.0_f64 * t2745 * t880 - 3.0_f64 * t2892 * t830 - 6.0_f64 * t10667 - 6.0_f64 * t10680 + 12.0_f64 * t10689 - 12.0_f64 * t10699 + 12.0_f64 * t10713 - 2.0_f64 * t10800 + 2.0_f64 * t10810;
    (t10808, t10810, t10818)
}
