//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 699/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk699<F: Float>(t1045: F, t5510: F, t1043: F, t1432: F, t2982: F, t2980: F, t3128: F, t5626: F, t3133: F, t1027: F, t1790: F, t1991: F, t1855: F, t3053: F, t3057: F, t3065: F, t3121: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8965 = t1045 * t5510;
    let t8966 = t1043 * t8965;
    let t8968 = t2982 * t1432;
    let t8969 = t2980 * t8968;
    let t8972 = t3128 * t5626;
    let t8974 = t3133 * t5626;
    let t8976 = t1027 * t1790;
    let t8978 = t1027 * t1991;
    let t8980 = t3053 * t1855;
    let t8982 = t3057 * t1855;
    let t8984 = t3121 * t3065;
    (t8965, t8966, t8969, t8972, t8974, t8976, t8978, t8980, t8982, t8984)
}
