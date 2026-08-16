//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1796/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1796(t10304: f64, t13537: f64, t136: f64, t2775: f64, t3966: f64, t607: f64) -> (f64, f64, f64) {
    let t13538 = t10304 * t13537;
    let t13539 = t136 * t13538;
    let t13541 = t2775 * t3966;
    let t13542 = t13541 * t607;
    (t13538, t13539, t13542)
}
