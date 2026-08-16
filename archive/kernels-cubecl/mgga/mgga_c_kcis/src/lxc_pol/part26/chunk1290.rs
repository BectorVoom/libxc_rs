//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1290/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1290<F: Float>(t2109: F, t833: F, t5440: F, t99198: F, t21863: F, t5661: F, t98034: F, t1307: F, t22758: F, t6159: F, t18210: F, t29549: F, t7978: F) -> (F, F, F, F, F) {
    let t102079 = t2109 * t833;
    let t102081 = t99198 * t5440 * t102079;
    let t102085 = t5661 * t98034 * t21863;
    let t102088 = t6159 * t22758 * t1307;
    let t102092 = t7978 * t18210 * t29549;
    (t102079, t102081, t102085, t102088, t102092)
}
