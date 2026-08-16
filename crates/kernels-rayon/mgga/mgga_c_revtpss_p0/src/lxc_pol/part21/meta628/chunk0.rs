//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2392/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2392(t2237: f64, t2482: f64, t849: f64, t2677: f64, t10489: f64, t221: f64, t2674: f64, t2675: f64, t234: f64, t9801: f64, t10887: f64, t136: f64, t2475: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40710 = t2482 * t849 * t2237;
    let t40711 = t40710 * t2677;
    let t40719 = t2674 * t2675 * t221 * t10489;
    let t40721 = t9801 * t234;
    let t40722 = t40721 * t10887;
    let t40724 = t2475 * t136;
    (t40710, t40711, t40719, t40721, t40722, t40724)
}
