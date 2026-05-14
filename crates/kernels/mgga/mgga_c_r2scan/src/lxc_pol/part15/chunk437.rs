//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 437/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk437<F: Float>(t681: F, t689: F, t1691: F, t226: F, t1399: F, t1732: F, t1734: F, t1738: F, t1740: F, t207: F, t1885: F, t1888: F, t1897: F, t1901: F, t1904: F, t1910: F, t1913: F, t1916: F, t1966: F, t202: F, t664: F, t687: F, t690: F, t718: F) -> (F, F, F, F, F) {
    let t2017 = t681 * t689;
    let t2021 = t226 * t1691;
    let t2029 = 0.235315e1 * t1732 - 0.62750666666666666667e1 * t1734 - 0.28051666666666666667e0 * t1738 + 0.56103333333333333335e0 * t1740 + 0.13892666666666666667e0 * t1399;
    let t2030 = t2029 * t207;
    let t2033 = 0.32163958997385070134e2 * t687 * t690 * t1966 + 0.64327917994770140268e2 * t687 * t2017 * t664 + 0.35089341735807877242e1 * t718 * t2021 + 1.0 * t202 * t2030 - t1885 - t1888 - t1897 - t1901 + t1904 + t1910 + t1913 - t1916;
    (t2017, t2021, t2029, t2030, t2033)
}
