//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 908/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk908(t173: f64, t5037: f64, t701: f64, t5041: f64, t3799: f64, t3803: f64, t227: f64, t4995: f64, t9: f64, t706: f64, t3814: f64, t13596: f64, t13601: f64, t13629: f64, t13636: f64, t13648: f64, t18032: f64, t9639: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18034 = t173 * t5037;
    let t18035 = t701 * t18034;
    let t18037 = t173 * t5041;
    let t18038 = t701 * t18037;
    let t18040 = t3799 * t3803;
    let t18043 = t9 * t227 * t4995;
    let t18044 = t18043 * t706;
    let t18046 = t3799 * t3814;
    let t18048 = -0.85124811172839506172e-2_f64 * t13596 + t13601 - 0.14187468528806584362e-2_f64 * t9639 - 0.85124811172839506172e-2_f64 * t13629 - t13636 - 0.28374937057613168724e-2_f64 * t13648 + 0.21281202793209876543e-2_f64 * t18032 + 0.28374937057613168724e-2_f64 * t18035 - 0.42562405586419753087e-2_f64 * t18038 - 0.1134997482304526749e-1_f64 * t18040 + 0.62424861526748971193e-1_f64 * t18044 + 0.6809984893827160494e-1_f64 * t18046;
    (t18035, t18038, t18040, t18044, t18046, t18048)
}
