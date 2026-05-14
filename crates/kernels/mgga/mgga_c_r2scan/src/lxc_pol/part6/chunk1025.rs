//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1025/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1025<F: Float>(t2572: F, t7944: F, t360: F, t6063: F, t7605: F, t2155: F, t551: F, t552: F, t7591: F, t5109: F, t7356: F, t2207: F, t2208: F, t2837: F, t2612: F, t495: F) -> (F, F, F, F, F, F, F, F) {
    let t7945 = t2572 * t7944;
    let t7946 = t360 * t7945;
    let t7949 = t6063 * t7605;
    let t7951 = 0.19514881078765566037e-1 * t2155 * t7949;
    let t7953 = t551 * t552 * t7591;
    let t7956 = t5109 * t7356;
    let t7961 = t2207 * t2837 * t2208;
    let t7963 = t2612 * t495;
    (t7945, t7946, t7949, t7951, t7953, t7956, t7961, t7963)
}
