//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1224/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1224<F: Float>(t2049: F, t597: F, t761: F, t6028: F, t5879: F, t759: F, t1783: F, t6044: F, t607: F, t18946: F, t18948: F, t18951: F, t18954: F, t22202: F, t166: F, t2288: F, t58: F) -> (F, F, F, F, F, F, F, F) {
    let t22625 = t597 * t761 * t2049;
    let t22626 = t6028 * t22625;
    let t22630 = t759 * t5879 * t761;
    let t22633 = t759 * t1783 * t2049;
    let t22636 = t759 * t607 * t6044;
    let t22644 = -0.24694444444444444444e-1 * t18946 + 0.23706666666666666666e0 * t18948 - 0.87802469135802469134e-1 * t18951 + 0.76827160493827160493e-1 * t18954 + t22202;
    let t22647 = 0.285764e-1 * t759 * t166 * t22644;
    let t22648 = t2288 * t58;
    (t22625, t22626, t22630, t22633, t22636, t22644, t22647, t22648)
}
