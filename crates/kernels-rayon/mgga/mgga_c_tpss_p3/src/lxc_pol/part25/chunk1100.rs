//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1100/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1100(t14694: f64, t14696: f64, t14698: f64, t14700: f64, t14703: f64, t14830: f64, t14860: f64, t14862: f64, t14865: f64, t14868: f64, t14871: f64, t14874: f64, t14878: f64, t14881: f64, t14885: f64, t14889: f64, t14892: f64, t14894: f64, t15202: f64, t198: f64, t330: f64, t4019: f64, t4023: f64, t4024: f64, t995: f64) -> f64 {
    let t15206 = t15202 * t198 * t330 * t995 - 2.0_f64 * t4019 * t4023 * t4024 - t14694 + t14696 - t14698 - t14700 + t14703 + t14830 - t14860 + t14862 + t14865 - t14868 - t14871 - t14874 + t14878 + t14881 + t14885 - t14889 - t14892 - t14894;
    t15206
}
