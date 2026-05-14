//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1301/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1301<F: Float>(t43982: F, t9364: F, t9368: F, t111062: F, t111065: F, t111067: F, t111069: F, t111071: F, t111073: F, t111075: F, t111078: F, t111081: F, t111085: F, t111048: F, t9365: F, t15492: F, t32636: F) -> (F, F, F) {
    let t111088 = t43982 * t9364 * t9368;
    let t111090 = 0.62500000000000000002e-1 * t111062 + 0.93819444444444444446e-1 * t111065 + 0.844375e-1 * t111067 - 0.36187500000000000001e-1 * t111069 - 0.36187500000000000001e-1 * t111071 - 0.62500000000000000002e-1 * t111073 - 0.62500000000000000002e-1 * t111075 + 0.62500000000000000002e-1 * t111078 + 0.65001222222222222219e-1 * t111081 + 0.10416666666666666667e-1 * t111085 + 0.120625e-1 * t111088;
    let t111091 = t9365 * t111048;
    let t111094 = t15492 * t32636 * t9368;
    (t111090, t111091, t111094)
}
