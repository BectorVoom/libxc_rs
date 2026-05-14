//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 925/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk925<F: Float>(t10918: F, t684: F, t664: F, t10769: F, t5812: F, t7357: F, t9148: F, t10870: F, t10873: F, t10878: F, t10887: F, t10891: F, t10894: F, t10896: F, t10898: F, t10900: F, t10903: F, t1108: F, t1938: F, t1977: F, t248: F, t2829: F, t3565: F, t3592: F, t3605: F, t3608: F, t5838: F, t7315: F, t7486: F, t7494: F, t9499: F) -> (F, F, F, F) {
    let t10919 = t10918 * t684;
    let t10921 = 1.0 * t664 * t10919;
    let t10925 = -t5812 + 0.68493333333333333332e-1 * t7357 - 0.51369999999999999999e-1 * t9148 + 0.5137e-1 * t10769;
    let t10928 = -t10870 - 0.35089341735807877242e1 * t7494 * t3592 + 0.35089341735807877242e1 * t1977 * t10873 - 6.0 * t7486 * t3565 + 6.0 * t1938 * t10878 + 0.17544670867903938621e1 * t9499 * t1108 + 0.17544670867903938621e1 * t2829 * t3605 + 0.51947577317044391276e2 * t7315 * t3608 - 0.10389515463408878255e3 * t5838 * t10887 + t10891 - t10894 - t10896 - t10898 - t10900 + t10903 - t10921 - 0.310907e-1 * t10925 * t248;
    (t10919, t10921, t10925, t10928)
}
