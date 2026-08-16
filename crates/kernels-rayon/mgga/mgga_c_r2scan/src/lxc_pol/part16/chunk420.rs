//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 420/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk420(t681: f64, t689: f64, t1691: f64, t226: f64, t1399: f64, t1732: f64, t1734: f64, t1738: f64, t1740: f64, t207: f64, t1885: f64, t1888: f64, t1897: f64, t1901: f64, t1904: f64, t1910: f64, t1913: f64, t1916: f64, t1966: f64, t202: f64, t664: f64, t687: f64, t690: f64, t718: f64) -> (f64, f64, f64, f64, f64) {
    let t2017 = t681 * t689;
    let t2021 = t226 * t1691;
    let t2029 = 0.235315e1_f64 * t1732 - 0.62750666666666666667e1_f64 * t1734 - 0.28051666666666666667e0_f64 * t1738 + 0.56103333333333333335e0_f64 * t1740 + 0.13892666666666666667e0_f64 * t1399;
    let t2030 = t2029 * t207;
    let t2033 = 0.32163958997385070134e2_f64 * t687 * t690 * t1966 + 0.64327917994770140268e2_f64 * t687 * t2017 * t664 + 0.35089341735807877242e1_f64 * t718 * t2021 + 1.0_f64 * t202 * t2030 - t1885 - t1888 - t1897 - t1901 + t1904 + t1910 + t1913 - t1916;
    (t2017, t2021, t2029, t2030, t2033)
}
