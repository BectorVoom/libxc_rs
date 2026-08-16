//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1369/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1369(t3362: f64, t414: f64, t66: f64, t42859: f64, t460: f64, t42865: f64, t479: f64, t1244: f64, t42871: f64, t471: f64, t12884: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44361 = 1.0_f64 / t414 / t3362;
    let t44362 = t66 * t44361;
    let t44372 = t460 * t42859;
    let t44373 = t479 * t42865;
    let t44375 = t44372 * t1244 * t44373;
    let t44378 = t42871 * t471;
    let t44425 = t828 * t12884;
    (t44362, t44372, t44373, t44375, t44378, t44425)
}
