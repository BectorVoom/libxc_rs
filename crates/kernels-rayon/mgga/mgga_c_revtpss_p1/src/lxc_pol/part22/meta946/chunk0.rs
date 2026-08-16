//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3183/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3183(t43766: f64, t44361: f64, t12916: f64, t17419: f64, t5340: f64, t45608: f64, t58919: f64, t45786: f64, t17708: f64, t45846: f64, t12975: f64, t1803: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t58983 = t44361 * t43766;
    let t58997 = t5340 * t12916 * t17419;
    let t59001 = t45608 * t58919;
    let t59011 = t45786 * t58919;
    let t59017 = t45846 * t17708;
    let t59025 = t12975 * t1803;
    (t58983, t58997, t59001, t59011, t59017, t59025)
}
