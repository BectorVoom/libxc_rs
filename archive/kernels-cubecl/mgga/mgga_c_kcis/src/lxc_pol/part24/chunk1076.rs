//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1076/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1076<F: Float>(t1267: F, t1646: F, t28110: F, t5310: F, t1071: F, t1268: F, t4547: F, t2844: F, t5302: F, t1262: F, t1856: F, t26996: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28111 = t1646 * t1267;
    let t28112 = t28110 * t28111;
    let t28113 = t5310 * t28112;
    let t28116 = t1268 * t1071;
    let t28117 = t28116 * t4547;
    let t28118 = t5310 * t28117;
    let t28123 = t1268 * t2844;
    let t28124 = t28123 * t4547;
    let t28125 = t5302 * t28124;
    let t28130 = t1856 * t1262;
    let t28131 = t26996 * t28130;
    (t28111, t28112, t28113, t28116, t28117, t28118, t28123, t28124, t28125, t28130, t28131)
}
