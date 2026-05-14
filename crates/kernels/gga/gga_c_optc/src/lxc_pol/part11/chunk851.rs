//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 851/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk851<F: Float>(t1: F, t17045: F, t297: F, t313: F, t16988: F, t7380: F, t935: F, t16225: F, t7405: F, t322: F, t7924: F, t16231: F, t865: F, t7397: F, t11008: F, t11074: F, t11111: F, t14313: F, t14327: F, t14421: F, t14426: F, t14431: F, t17024: F, t17028: F, t17031: F, t17035: F, t3835: F, t7379: F, t7447: F, t7449: F, t7491: F, t7897: F, t862: F, t874: F) -> (F, F, F, F, F, F, F) {
    let t17047 = t17045 * t1 * t297;
    let t17048 = t313 * t17047;
    let t17052 = t16988 * t7380 * t935;
    let t17053 = t313 * t17052;
    let t17056 = t7405 * t16225;
    let t17057 = t322 * t17056;
    let t17060 = t7924 * t16225;
    let t17061 = t322 * t17060;
    let t17064 = t865 * t16231;
    let t17065 = t322 * t17064;
    let t17068 = t7397 * t16225;
    let t17069 = t322 * t17068;
    let t17078 = 0.18314556960919660338e2 * t14313 + t7447 + 0.90553765518632894319e-2 * t3835 * t17024 + 0.71000632978163088351e-1 * t14327 - 0.10866451862235947318e-1 * t3835 * t17028 - 0.91572784804598301689e1 * t7449 * t17031 + 0.18314556960919660338e2 * t7491 * t17035 + 0.35500316489081544176e-1 * t874 * t17048 + 0.10629507243271336419e5 * t7379 * t17053 - t862 * t17057 / 36.0 + 7.0 / 648.0 * t862 * t17061 + t862 * t17065 / 288.0 + t862 * t17069 / 48.0 - t11008 / 432.0 - t14421 / 144.0 + t14426 / 288.0 + t14431 / 216.0 + t7897 - 0.12073835402484385909e-2 * t11074 - 0.23666877659387696117e-1 * t11111;
    (t17047, t17052, t17056, t17060, t17064, t17068, t17078)
}
