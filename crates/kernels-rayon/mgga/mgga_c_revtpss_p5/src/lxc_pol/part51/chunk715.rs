//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 715/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk715(t225: f64, t7910: f64, t1892: f64, t1955: f64, t1903: f64, t2022: f64, t7296: f64, t1882: f64, t543: f64, t7301: f64, t545: f64, t2028: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7911 = t7910 * t225;
    let t7917 = t1955 * t1892;
    let t7920 = t2022 * t1903;
    let t7921 = t7296 * t7920;
    let t7925 = t2022 * t1882 * t543;
    let t7926 = t7301 * t7925;
    let t7929 = t545 * t7910;
    let t7930 = t2028 * t7929;
    (t7911, t7917, t7920, t7921, t7925, t7926, t7929, t7930)
}
