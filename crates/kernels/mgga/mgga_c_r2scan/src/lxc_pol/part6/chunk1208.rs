//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1208/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1208<F: Float>(t1762: F, t1978: F, t5916: F, t1734: F, t5951: F, t5954: F, t1942: F, t5215: F, t1726: F, t1871: F, t377: F, t5: F, t591: F, t611: F, t614: F, t1732: F) -> (F, F, F, F, F, F) {
    let t22270 = 0.12842595503380418954e1 * t1762 * t5916 * t1978;
    let t22271 = t5951 * t1734;
    let t22274 = t5954 * t1734;
    let t22278 = 0.86748650402413918737e-1 * t1762 * t5215 * t1942;
    let t22285 = 0.6097060704e-1 * t1726 * t611 * t614 * t5 * t377 * t1871 * t591;
    let t22286 = t5954 * t1732;
    (t22270, t22271, t22274, t22278, t22285, t22286)
}
