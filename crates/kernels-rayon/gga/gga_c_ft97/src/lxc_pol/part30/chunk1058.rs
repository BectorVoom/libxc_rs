//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1058/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1058(t1403: f64, t35296: f64, t681: f64, t27968: f64, t7437: f64, t150036: f64, t150040: f64, t150044: f64, t150047: f64, t150051: f64, t150054: f64, t150058: f64, t150062: f64, t150066: f64, t150069: f64, t150073: f64, t150077: f64, t150079: f64, t150084: f64, t150088: f64, t150092: f64) -> (f64, f64, f64) {
    let t151200 = t1403 * t681 * t35296;
    let t151212 = t7437 * t27968;
    let t151230 = 2.0_f64 / 27.0_f64 * t150036 - t150040 / 9.0_f64 + t150044 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t150047 - t150051 / 3.0_f64 - 2.0_f64 * t150054 + t150058 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t150062 + 2.0_f64 / 9.0_f64 * t150066 - 2.0_f64 / 27.0_f64 * t150069 + t150073 / 18.0_f64 + t150077 / 18.0_f64 - t150079 / 54.0_f64 + t150084 / 18.0_f64 - t150088 / 9.0_f64 + t150092 / 2.0_f64;
    (t151200, t151212, t151230)
}
