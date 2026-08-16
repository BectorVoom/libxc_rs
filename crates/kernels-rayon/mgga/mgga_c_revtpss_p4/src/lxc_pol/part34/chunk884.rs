//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 884/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk884(t6071: f64, t72: f64, t686: f64, t2465: f64, t213: f64, t6041: f64, t6048: f64, t10995: f64, t6072: f64, t779: f64, t689: f64, t1580: f64, t4321: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18796 = t6071 * t72;
    let t18797 = t18796 * t686;
    let t18798 = t2465 * t18797;
    let t18800 = t213 * t6041;
    let t18804 = t6048 * t72;
    let t18805 = t18804 * t686;
    let t18806 = t10995 * t18805;
    let t18811 = t779 * t6072;
    let t18812 = t689 * t18811;
    let t18814 = t4321 * t1580;
    (t18797, t18798, t18800, t18805, t18806, t18812, t18814)
}
