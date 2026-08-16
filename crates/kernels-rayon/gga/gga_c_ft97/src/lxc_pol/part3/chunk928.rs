//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 928/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk928(t18302: f64, t18360: f64, t734: f64, t91: f64, t2475: f64, t5120: f64, t747: f64, t13739: f64, t13983: f64, t13984: f64, t13993: f64, t13998: f64, t14004: f64, t18142: f64, t18265: f64, t18266: f64) -> (f64, f64, f64) {
    let t18361 = t18302 + t18360;
    let t18363 = t91 * t734 * t18361;
    let t18365 = t2475 * t5120;
    let t18367 = t91 * t18365 * t747;
    let t18369 = -8.0_f64 / 9.0_f64 * t13739 - t13983 + t13984 - t18142 - t18265 + t18266 - t13993 + t13998 - t14004 + t18363 / 2.0_f64 - t18367 / 4.0_f64;
    (t18363, t18367, t18369)
}
