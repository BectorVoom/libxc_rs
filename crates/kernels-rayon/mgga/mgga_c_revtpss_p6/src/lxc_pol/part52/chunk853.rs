//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 853/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk853(t25460: f64, t7150: f64, t11120: f64, t359: f64, t1982: f64, t994: f64, t1972: f64, t3223: f64, t1024: f64, t7125: f64, t3215: f64, t7117: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25461 = t7150 * t25460;
    let t25464 = t11120 * t359;
    let t25473 = t1982 * t25460;
    let t25476 = t994 * t25460;
    let t25490 = t3223 * t1972;
    let t25495 = t1024 * t7125;
    let t25498 = t7117 * t3215;
    (t25461, t25464, t25473, t25476, t25490, t25495, t25498)
}
