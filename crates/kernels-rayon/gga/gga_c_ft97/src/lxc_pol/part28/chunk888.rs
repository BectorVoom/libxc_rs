//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 888/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk888(t13140: f64, t35067: f64, t1901: f64, t33008: f64, t33016: f64, t33066: f64, t35035: f64, t35039: f64, t35043: f64, t35047: f64, t35052: f64, t35056: f64, t35060: f64, t35064: f64, t446: f64) -> (f64, f64) {
    let t35068 = t13140 * t35067;
    let t35071 = t33008 + 4.0_f64 / 3.0_f64 * t446 * t35035 + 4.0_f64 / 3.0_f64 * t446 * t35039 + 2.0_f64 / 3.0_f64 * t446 * t35043 + t33016 - t446 * t35047 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t35052 + 4.0_f64 / 3.0_f64 * t446 * t35056 + 2.0_f64 / 3.0_f64 * t446 * t35060 - t33066 + t1901 * t35064 / 9.0_f64 - 4.0_f64 / 3.0_f64 * t1901 * t35068;
    (t35068, t35071)
}
