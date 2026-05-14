//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 719/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk719<F: Float>(t3128: F, t5626: F, t3133: F, t1027: F, t1790: F, t1991: F, t1855: F, t3053: F, t3057: F, t3065: F, t3121: F, t1971: F, t3707: F, t1030: F, t3076: F, t1795: F, t3104: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8972 = t3128 * t5626;
    let t8974 = t3133 * t5626;
    let t8976 = t1027 * t1790;
    let t8978 = t1027 * t1991;
    let t8980 = t3053 * t1855;
    let t8982 = t3057 * t1855;
    let t8984 = t3121 * t3065;
    let t8986 = t1971 * t3707;
    let t8987 = t1030 * t8986;
    let t8988 = t8987 * t3076;
    let t8990 = t3104 * t1795;
    (t8972, t8974, t8976, t8978, t8980, t8982, t8984, t8986, t8987, t8988, t8990)
}
