//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1284/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1284(t17946: f64, t3622: f64, t17960: f64, t3667: f64, t17974: f64, t3685: f64, t19695: f64, t19697: f64, t5543: f64, t136: f64, t1693: f64, t799: f64) -> (f64, f64, f64, f64, f64) {
    let t63966 = t17946 * t3622;
    let t63973 = t17960 * t3667;
    let t63977 = t17974 * t3685;
    let t63990 = t5543 * t19695 * t19697;
    let t63993 = t1693 * t799 * t136;
    (t63966, t63973, t63977, t63990, t63993)
}
