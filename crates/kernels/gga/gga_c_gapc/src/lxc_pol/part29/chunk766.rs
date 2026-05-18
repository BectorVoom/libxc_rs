//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 766/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk766<F: Float>(t3128: F, t5626: F, t3133: F, t1027: F, t1790: F, t1991: F, t1855: F, t3053: F, t3057: F, t3065: F, t3121: F, t1971: F, t3707: F) -> (F, F, F, F, F, F, F, F) {
    let t8972 = t3128 * t5626;
    let t8974 = t3133 * t5626;
    let t8976 = t1027 * t1790;
    let t8978 = t1027 * t1991;
    let t8980 = t3053 * t1855;
    let t8982 = t3057 * t1855;
    let t8984 = t3121 * t3065;
    let t8986 = t1971 * t3707;
    (t8972, t8974, t8976, t8978, t8980, t8982, t8984, t8986)
}
