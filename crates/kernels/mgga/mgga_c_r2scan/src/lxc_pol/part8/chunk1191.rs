//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1191/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1191<F: Float>(t2312: F, t2315: F, t2316: F, t158: F, t166: F, t2049: F, t2289: F, t23059: F, t23063: F, t23067: F, t23128: F, t2317: F, t23179: F, t6044: F, t6806: F, t6868: F, t6876: F, t874: F, t875: F) -> (F,) {
    let t23189 = t2312 * t2312;
    let t23192 = t2315 * t2315;
    let t23193 = t2316 * t2316;
    let t23194 = 1.0 / t23193;
    let t23199 = (-0.6858336e0 * t23059 + 0.10287504e1 * t2289 * t2049 - 0.1714584e0 * t23063 - 0.2286112e0 * t6806 * t6044 + 0.285764e-1 * t23067 + 0.285764e-1 * (t23128 + t23179) * t875 - 0.1143056e0 * t6868 * t2317 * t874 + 0.3429168e0 * t2312 * t6876 * t2315 - 0.857292e-1 * t23189 * t2317 - 0.1714584e0 * t23192 * t23194) * t158 * t166;
    (t23199,)
}
