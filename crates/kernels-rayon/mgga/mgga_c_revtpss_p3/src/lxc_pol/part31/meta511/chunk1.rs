//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1851/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1851(t27440: f64, t7160: f64, t7810: f64, t988: f64, t7145: f64, t4820: f64, t7122: f64, t4878: f64, t7121: f64) -> (f64, f64, f64, f64) {
    let t27441 = t7160 * t27440;
    let t27444 = t7810 * t988;
    let t27445 = t7145 * t27444;
    let t27448 = t7122 * t4820;
    let t27450 = t4878 * t7121;
    (t27441, t27445, t27448, t27450)
}
