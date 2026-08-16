//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2915/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2915(t15547: f64, t3026: f64, t11616: f64, t4719: f64, t11598: f64, t11507: f64, t300: f64, t15266: f64, t52239: f64, t11591: f64, t4725: f64, t15556: f64, t3022: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52885 = 0.35089341735807877242e1_f64 * t15547 * t3026;
    let t52887 = 0.10254018858216406658e4_f64 * t4719 * t11616;
    let t52889 = 0.35089341735807877242e1_f64 * t4719 * t11598;
    let t52894 = t300 * t11507;
    let t52897 = 0.30762056574649219974e4_f64 * t52894 * t15266 * t52239;
    let t52899 = 0.35089341735807877242e1_f64 * t11591 * t4725;
    let t52905 = 0.51947577317044391277e2_f64 * t3022 * t15556;
    (t52885, t52887, t52889, t52897, t52899, t52905)
}
