//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 943/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk943<F: Float>(t13150: F, t4555: F, t3210: F, t4554: F, t1774: F, t9568: F, t1092: F, t3178: F, t5019: F, t3198: F, t4992: F, t86: F, t167: F, t829: F, t4546: F, t3183: F, t4999: F) -> (F, F, F, F, F, F, F) {
    let t13161 = t4555 * t13150;
    let t13162 = t3210 * t13161;
    let t13163 = t4554 * t13162;
    let t13165 = t9568 * t1774;
    let t13166 = t1092 * t13165;
    let t13168 = t3178 * t5019;
    let t13169 = t1092 * t13168;
    let t13172 = t86 * t4992 * t3198;
    let t13173 = t167 * t829;
    let t13174 = t4546 * t13173;
    let t13175 = t3210 * t13174;
    let t13176 = t13172 * t13175;
    let t13178 = t4999 * t3183;
    (t13163, t13166, t13169, t13172, t13173, t13176, t13178)
}
