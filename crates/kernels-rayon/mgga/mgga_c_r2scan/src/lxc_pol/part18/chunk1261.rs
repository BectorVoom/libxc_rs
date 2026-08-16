//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1261/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1261(t1044: f64, t12019: f64, t12703: f64, t354: f64, t43818: f64, t43823: f64, t43834: f64, t43844: f64, t43851: f64, t43864: f64, t43867: f64, t43870: f64, t43883: f64, t43889: f64, t43909: f64, t43914: f64, t43924: f64, t43929: f64, t43932: f64, t43946: f64, t43949: f64, t43953: f64, t43958: f64, t43962: f64, t43963: f64, t43968: f64, t43971: f64, t43974: f64, t43976: f64, t43978: f64, t43982: f64, t43987: f64, t43991: f64, t43992: f64, t885: f64) -> f64 {
    let t44001 = t354 * (t43818 + t43823 + t43834 + t43844 + t43851 + t43864 + t43867 + t43870 + t43883 + t43889 + t43909 + t43914 + t43924 + t43932 + t43963 + t43992) - t43929 + t43946 + t43949 + t43953 + t43958 + t43962 + 2.0_f64 * t12019 * t1044 + t43968 - t43971 - t43974 + t12703 * t885 + t43976 - t43978 + t43982 - t43987 + t43991;
    t44001
}
