//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 425/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk425<F: Float>(t1670: F, t932: F, t939: F, t1662: F, t945: F, t26: F, t1664: F, t937: F, t944: F, t950: F) -> (F, F, F, F, F, F) {
    let t1671 = t932 * t1670;
    let t1674 = t939 * t1670;
    let t1676 = t945 * t1662;
    let t1677 = t26 * t1676;
    let t1679 = 0.1898925e1 * t1671 - t937 - 0.29896666666666666667e0 * t1664 + 0.3071625e0 * t1674 - t944 - 0.82156666666666666667e-1 * t1677;
    let t1680 = t1679 * t950;
    (t1671, t1674, t1676, t1677, t1679, t1680)
}
