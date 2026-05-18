//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1043/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1043<F: Float>(t167: F, t829: F, t4546: F, t3210: F, t13172: F, t3183: F, t4999: F, t1092: F, t5168: F, t1134: F, t1800: F, t2850: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t13173 = t167 * t829;
    let t13174 = t4546 * t13173;
    let t13175 = t3210 * t13174;
    let t13176 = t13172 * t13175;
    let t13178 = t4999 * t3183;
    let t13179 = t1092 * t13178;
    let t13181 = t5168 * sigma0;
    let t13182 = t13181 * t1134;
    let t13183 = t1092 * t13182;
    let t13186 = t1800 * t2850;
    (t13173, t13176, t13179, t13181, t13183, t13186)
}
