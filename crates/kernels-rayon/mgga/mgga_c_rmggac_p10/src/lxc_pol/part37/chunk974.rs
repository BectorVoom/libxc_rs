//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 974/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk974(t77613: f64, t1494: f64, t1970: f64, t1971: f64, t209: f64, t515: f64, t698: f64, t75446: f64, t75448: f64, t75450: f64, t75452: f64, t75465: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77614 = 0.42564599893297839398e-5_f64 * t77613;
    let t77619 = t1970 * t1971 * t515 * t698 * t1494 * t209;
    let t77620 = 0.42564599893297839398e-5_f64 * t77619;
    let t77621 = 0.86737941314158990619e-4_f64 * t75446;
    let t77624 = 0.68186654135613354325e-2_f64 * t75448;
    let t77625 = 0.20455996240684006296e-1_f64 * t75450;
    let t77626 = 0.40911992481368012592e-1_f64 * t75452;
    let t77630 = 0.2553875993597870364e-4_f64 * t75465;
    (t77614, t77620, t77621, t77624, t77625, t77626, t77630)
}
