//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1181/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1181<F: Float>(t406: F, t5845: F, t1654: F, t2049: F, t597: F, t6044: F, t2061: F, t2056: F, t6006: F, t6007: F, t607: F, t761: F, t6028: F, t759: F, t18946: F, t18948: F, t18951: F, t18954: F, t22202: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22575 = t406 * t5845;
    let t22595 = t1654 * t2049;
    let t22602 = t597 * t6044;
    let t22603 = t2061 * t22602;
    let t22608 = t1654 * t2056;
    let t22616 = t6006 * t607 * t6007;
    let t22625 = t597 * t761 * t2049;
    let t22626 = t6028 * t22625;
    let t22636 = t759 * t607 * t6044;
    let t22644 = -0.24694444444444444444e-1 * t18946 + 0.23706666666666666666e0 * t18948 - 0.87802469135802469134e-1 * t18951 + 0.76827160493827160493e-1 * t18954 + t22202;
    (t22575, t22595, t22602, t22603, t22608, t22616, t22625, t22626, t22636, t22644)
}
