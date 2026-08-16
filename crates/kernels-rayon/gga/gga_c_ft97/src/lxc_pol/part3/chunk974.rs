//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 974/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk974(t19048: f64, t4092: f64, t19043: f64, t811: f64, t14722: f64, t1208: f64, t14752: f64, t4064: f64, t4125: f64, t820: f64, t14738: f64, t2724: f64, t5284: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19053 = t4092 * t19048;
    let t19056 = t19043 * t811;
    let t19057 = t14722 * t19056;
    let t19066 = t14752 * t1208;
    let t19069 = t4064 * t4125;
    let t19072 = t1208 * t820;
    let t19073 = t14738 * t19072;
    let t19076 = t2724 * t5284;
    (t19053, t19057, t19066, t19069, t19073, t19076)
}
