//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1077/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1077(t30: f64, t7782: f64, t33: f64, t196: f64, t197: f64, t7894: f64, t1883: f64, t32195: f64, t5673: f64, t32194: f64, t1868: f64, t3936: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33740 = t30 * t7782;
    let t33888 = t33 * t7782;
    let t33913 = t7894 * t196 * t197;
    let t33922 = t5673 * t32195 * t1883;
    let t33923 = t32194 * t33922;
    let t33926 = t3936 * t32195 * t1868;
    (t33740, t33888, t33913, t33922, t33923, t33926)
}
