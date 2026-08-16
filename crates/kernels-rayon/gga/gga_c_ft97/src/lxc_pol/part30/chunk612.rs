//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 612/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk612(t2506: f64, t27878: f64, t1434: f64, t193: f64, t2371: f64, t6837: f64, t713: f64, t89: f64, t27844: f64, t27848: f64, t27853: f64, t27858: f64, t27861: f64, t27864: f64, t27867: f64, t27870: f64, t27873: f64, t27876: f64) -> (f64, f64, f64, f64, f64) {
    let t27879 = t2506 * t27878;
    let t27881 = t1434 * t193 * t27879;
    let t27882 = t2371 * t6837;
    let t27883 = t27882 * t713;
    let t27884 = t193 * t27883;
    let t27885 = t89 * t27884;
    let t27887 = t27844 + t27848 / 4.0_f64 + t27853 / 4.0_f64 + t27858 / 4.0_f64 - 2.0_f64 / 3.0_f64 * t27861 - 2.0_f64 / 3.0_f64 * t27864 - 2.0_f64 / 3.0_f64 * t27867 + 2.0_f64 / 9.0_f64 * t27870 - t27873 / 12.0_f64 - t27876 / 3.0_f64 + t27881 + 2.0_f64 * t27885;
    (t27881, t27882, t27884, t27885, t27887)
}
