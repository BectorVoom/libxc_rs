//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1024/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1024(t19249: f64, t19252: f64, t19255: f64, t19258: f64, t19261: f64, t19265: f64, t19269: f64, t19754: f64, t19757: f64, t19761: f64, t19838: f64, t19278: f64) -> (f64, f64) {
    let t19839 = 2.0_f64 / 3.0_f64 * t19249;
    let t19849 = t19838 - t19839 - t19754 / 4.0_f64 - t19757 / 2.0_f64 + 3.0_f64 / 8.0_f64 * t19761 + 2.0_f64 / 3.0_f64 * t19252 - 2.0_f64 / 9.0_f64 * t19255 - 10.0_f64 / 27.0_f64 * t19258 + 8.0_f64 / 9.0_f64 * t19261 + 2.0_f64 / 3.0_f64 * t19265 - 4.0_f64 / 3.0_f64 * t19269;
    let t19852 = 2.0_f64 / 9.0_f64 * t19278;
    (t19849, t19852)
}
