//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1796/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1796(t122: f64, t2022: f64, t72: f64, t3916: f64, t25878: f64, t2435: f64, t7243: f64, t555: f64, t786: f64) -> (f64, f64, f64, f64, f64) {
    let t25880 = t2022 * t72 * t122;
    let t25881 = t25880 * t3916;
    let t25882 = t25878 * t25881;
    let t25893 = 0.73171657588172351096e-2_f64 * t2435 * t7243;
    let t25894 = t786 * t555;
    (t25880, t25881, t25882, t25893, t25894)
}
