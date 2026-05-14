//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 554/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk554<F: Float>(t1885: F, t1888: F, t1897: F, t1901: F, t1904: F, t1910: F, t1913: F, t1916: F, t1966: F, t2017: F, t202: F, t2021: F, t2030: F, t664: F, t687: F, t690: F, t718: F) -> (F,) {
    let t2033 = 0.32163958997385070134e2 * t687 * t690 * t1966 + 0.64327917994770140268e2 * t687 * t2017 * t664 + 0.35089341735807877242e1 * t718 * t2021 + 1.0 * t202 * t2030 - t1885 - t1888 - t1897 - t1901 + t1904 + t1910 + t1913 - t1916;
    (t2033,)
}
