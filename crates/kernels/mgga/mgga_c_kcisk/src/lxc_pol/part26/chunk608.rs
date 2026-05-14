//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 608/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk608<F: Float>(t1237: F, t6043: F, t4011: F, t4041: F, t6020: F, t6023: F, t6026: F, t6030: F, t1235: F, t2119: F, t4054: F, t1242: F, t1248: F, t2075: F, t3979: F, t4065: F, t5671: F) -> (F, F, F, F, F, F, F, F) {
    let t6044 = t6043 * t1237;
    let t6051 = t4041 + t4011 / 9.0 + t6020 / 9.0 - 2.0 / 9.0 * t6023 + 2.0 / 3.0 * t6026 - 2.0 / 3.0 * t6030;
    let t6052 = t1235 * t6051;
    let t6059 = t4054 * t2119;
    let t6060 = t6059 * t1237;
    let t6062 = t1242 * t6051;
    let t6066 = t1248 * t3979 * t2075;
    let t6069 = t1248 * t4065 * t5671;
    (t6044, t6051, t6052, t6059, t6060, t6062, t6066, t6069)
}
