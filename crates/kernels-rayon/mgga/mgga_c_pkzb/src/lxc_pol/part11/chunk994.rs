//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 994/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk994(t10918: f64, t684: f64, t664: f64, t10769: f64, t5812: f64, t7357: f64, t9148: f64, t10870: f64, t10873: f64, t10878: f64, t10887: f64, t10891: f64, t10894: f64, t10896: f64, t10898: f64, t10900: f64, t10903: f64, t1108: f64, t1938: f64, t1977: f64, t248: f64, t2829: f64, t3565: f64, t3592: f64, t3605: f64, t3608: f64, t5838: f64, t7315: f64, t7486: f64, t7494: f64, t9499: f64) -> (f64, f64, f64, f64) {
    let t10919 = t10918 * t684;
    let t10921 = 1.0_f64 * t664 * t10919;
    let t10925 = -t5812 + 0.68493333333333333332e-1_f64 * t7357 - 0.51369999999999999999e-1_f64 * t9148 + 0.5137e-1_f64 * t10769;
    let t10928 = -t10870 - 0.35089341735807877242e1_f64 * t7494 * t3592 + 0.35089341735807877242e1_f64 * t1977 * t10873 - 6.0_f64 * t7486 * t3565 + 6.0_f64 * t1938 * t10878 + 0.17544670867903938621e1_f64 * t9499 * t1108 + 0.17544670867903938621e1_f64 * t2829 * t3605 + 0.51947577317044391276e2_f64 * t7315 * t3608 - 0.10389515463408878255e3_f64 * t5838 * t10887 + t10891 - t10894 - t10896 - t10898 - t10900 + t10903 - t10921 - 0.310907e-1_f64 * t10925 * t248;
    (t10919, t10921, t10925, t10928)
}
