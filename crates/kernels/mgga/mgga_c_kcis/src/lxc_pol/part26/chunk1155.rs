//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1155/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1155<F: Float>(t2109: F, t833: F, t5440: F, t99198: F, t21863: F, t5661: F, t98034: F, t1307: F, t22758: F, t6159: F, t18210: F, t29549: F, t7978: F, t27567: F, t27583: F, t29575: F, t94928: F, t94974: F, t94977: F, t99176: F, t99193: F, t99229: F, t99238: F) -> (F, F, F, F, F) {
    let t102079 = t2109 * t833;
    let t102081 = t99198 * t5440 * t102079;
    let t102085 = t5661 * t98034 * t21863;
    let t102088 = t6159 * t22758 * t1307;
    let t102092 = t7978 * t18210 * t29549;
    let t102098 = -0.15445601851851851852e-3 * t99176 + t99193 - 0.46336805555555555556e-3 * t27583 * t102081 + 0.25794135802469135802e-2 * t102085 + 0.15459116753472222222e-4 * t27567 * t102088 + 0.11584201388888888889e-3 * t102092 + t99229 + 0.23168402777777777778e-3 * t94928 * t29575 + t99238 - 0.7722800925925925926e-4 * t94974 - 0.7722800925925925926e-4 * t94977;
    (t102079, t102081, t102085, t102088, t102098)
}
