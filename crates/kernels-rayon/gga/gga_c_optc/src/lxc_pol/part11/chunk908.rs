//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 908/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk908(t17064: f64, t322: f64, t16225: f64, t7397: f64, t11008: f64, t11074: f64, t11111: f64, t14313: f64, t14327: f64, t14421: f64, t14426: f64, t14431: f64, t17024: f64, t17028: f64, t17031: f64, t17035: f64, t17048: f64, t17053: f64, t17057: f64, t17061: f64, t3835: f64, t7379: f64, t7447: f64, t7449: f64, t7491: f64, t7897: f64, t862: f64, t874: f64) -> (f64, f64) {
    let t17065 = t322 * t17064;
    let t17068 = t7397 * t16225;
    let t17069 = t322 * t17068;
    let t17078 = 0.18314556960919660338e2_f64 * t14313 + t7447 + 0.90553765518632894319e-2_f64 * t3835 * t17024 + 0.71000632978163088351e-1_f64 * t14327 - 0.10866451862235947318e-1_f64 * t3835 * t17028 - 0.91572784804598301689e1_f64 * t7449 * t17031 + 0.18314556960919660338e2_f64 * t7491 * t17035 + 0.35500316489081544176e-1_f64 * t874 * t17048 + 0.10629507243271336419e5_f64 * t7379 * t17053 - t862 * t17057 / 36.0_f64 + 7.0_f64 / 648.0_f64 * t862 * t17061 + t862 * t17065 / 288.0_f64 + t862 * t17069 / 48.0_f64 - t11008 / 432.0_f64 - t14421 / 144.0_f64 + t14426 / 288.0_f64 + t14431 / 216.0_f64 + t7897 - 0.12073835402484385909e-2_f64 * t11074 - 0.23666877659387696117e-1_f64 * t11111;
    (t17068, t17078)
}
