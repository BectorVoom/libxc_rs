//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 479/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk479<F: Float>(t1564: F, t1577: F, t1585: F, t187: F, t1895: F, t1909: F, t1912: F, t1921: F, t2072: F, t2080: F, t2084: F, t601: F, t1928: F, t20: F, t610: F, t1601: F, t1650: F) -> (F, F, F, F) {
    let t2093 = -t1895 + t1909 + t187 * (-0.3109e-1 * t2072 * t601 + 1.0 * t1564 * t2080 + t1895 - t1909 - 0.19751789702565206229e-1 * t1912 + 0.58482233974552040708e0 * t1577 * t2084) + 0.19751789702565206229e-1 * t187 * t1912 - 0.58482233974552040708e0 * t1585 * t1921;
    let t2095 = t1928 * t20;
    let t2096 = t610 * t2095;
    let t2099 = t1601 * t1650;
    (t2093, t2095, t2096, t2099)
}
