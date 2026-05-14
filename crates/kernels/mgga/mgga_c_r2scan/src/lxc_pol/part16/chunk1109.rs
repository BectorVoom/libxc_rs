//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1109/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1109<F: Float>(t1044: F, t12019: F, t12703: F, t354: F, t43818: F, t43823: F, t43834: F, t43844: F, t43851: F, t43864: F, t43867: F, t43870: F, t43883: F, t43889: F, t43909: F, t43914: F, t43924: F, t43929: F, t43932: F, t43946: F, t43949: F, t43953: F, t43958: F, t43962: F, t43963: F, t43968: F, t43971: F, t43974: F, t43976: F, t43978: F, t43982: F, t43987: F, t43991: F, t43992: F, t885: F) -> (F,) {
    let t44001 = t354 * (t43818 + t43823 + t43834 + t43844 + t43851 + t43864 + t43867 + t43870 + t43883 + t43889 + t43909 + t43914 + t43924 + t43932 + t43963 + t43992) - t43929 + t43946 + t43949 + t43953 + t43958 + t43962 + 2.0 * t12019 * t1044 + t43968 - t43971 - t43974 + t12703 * t885 + t43976 - t43978 + t43982 - t43987 + t43991;
    (t44001,)
}
