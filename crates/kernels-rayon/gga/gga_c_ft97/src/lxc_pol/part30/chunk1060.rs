//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1060/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1060(t140784: f64, t140795: f64, t140797: f64, t141577: f64, t150154: f64, t150158: f64, t150162: f64, t150165: f64, t150168: f64, t150171: f64, t150175: f64, t150179: f64, t150184: f64, t150188: f64, t150194: f64, t150199: f64) -> f64 {
    let t151264 = -2.0_f64 * t150154 - 4.0_f64 * t150158 - 2.0_f64 / 9.0_f64 * t150162 + t150165 / 18.0_f64 + 4.0_f64 / 9.0_f64 * t150168 - t150171 / 9.0_f64 - t150175 / 36.0_f64 - t150179 / 36.0_f64 - 20.0_f64 / 3.0_f64 * t150184 + 8.0_f64 / 3.0_f64 * t150188 - t141577 + t140784 / 18.0_f64 + 2.0_f64 / 27.0_f64 * t140795 - t140797 / 27.0_f64 + 4.0_f64 / 9.0_f64 * t150194 + t150199 / 12.0_f64;
    t151264
}
