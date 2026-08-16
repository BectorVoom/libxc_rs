//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 999/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk999(t150081: f64, t2354: f64, t6118: f64, t684: f64, t27805: f64, t33341: f64, t3746: f64, t10157: f64, t24437: f64, t27814: f64, t33319: f64, t150036: f64, t150040: f64, t150044: f64, t150047: f64, t150051: f64, t150054: f64, t150058: f64, t150062: f64, t150066: f64, t150069: f64, t150073: f64, t150077: f64, t150079: f64) -> (f64, f64, f64, f64) {
    let t150084 = t6118 * t2354 * t150081 * t684;
    let t150088 = t27805 * t2354 * t33341 * t3746;
    let t150092 = t24437 * t10157 * t33319 * t27814;
    let t150094 = 2.0_f64 / 9.0_f64 * t150036 - t150040 / 3.0_f64 + t150044 - 2.0_f64 / 3.0_f64 * t150047 - t150051 - 6.0_f64 * t150054 + t150058 - 2.0_f64 / 3.0_f64 * t150062 + 2.0_f64 / 3.0_f64 * t150066 - 2.0_f64 / 9.0_f64 * t150069 + t150073 / 6.0_f64 + t150077 / 6.0_f64 - t150079 / 18.0_f64 + t150084 / 6.0_f64 - t150088 / 3.0_f64 + 3.0_f64 / 2.0_f64 * t150092;
    (t150084, t150088, t150092, t150094)
}
