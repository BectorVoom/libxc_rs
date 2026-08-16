//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 978/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk978(t28875: f64, t28887: f64, t545: f64, t2028: f64, t689: f64, t8099: f64, t25904: f64, t25899: f64, t213: f64, t8085: f64, t1904: f64, t7492: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28888 = t28875 + t28887;
    let t28889 = t545 * t28888;
    let t28890 = t2028 * t28889;
    let t28894 = t8099 * t689;
    let t28895 = t25904 * t28894;
    let t28897 = t25899 * t28894;
    let t28899 = t213 * t8085;
    let t28902 = t7492 * t1904;
    (t28888, t28890, t28895, t28897, t28899, t28902)
}
