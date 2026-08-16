//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 634/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk634<F: Float>(t1386: F, t7091: F, t1650: F, t2001: F, t4163: F, t4162: F, t4160: F, t556: F, t7053: F, t553: F, t303: F, t1983: F, t2006: F) -> (F, F, F, F, F, F, F, F) {
    let t7092 = t7091 * t1386;
    let t7099 = t1650 * t2001;
    let t7100 = t4163 * t7099;
    let t7101 = t4162 * t7100;
    let t7102 = t4160 * t7101;
    let t7104 = t7053 * t556;
    let t7105 = t553 * t7104;
    let t7106 = t303 * t7105;
    let t7108 = t1983 * t2006;
    (t7092, t7100, t7101, t7102, t7104, t7105, t7106, t7108)
}
