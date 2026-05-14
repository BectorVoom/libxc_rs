//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1128/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1128<F: Float>(t2649: F, t7245: F, t2643: F, t7255: F, t2742: F, t2754: F, t2751: F, t460: F, t458: F, t496: F, t2750: F, t457: F, t2747: F, t2757: F, t1101: F, t7544: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22186 = t7245 * t2649;
    let t22189 = t2643 * t7255;
    let t22191 = t2754 * t2742;
    let t22193 = t2751 * t2742;
    let t22195 = t460 * t460;
    let t22199 = 840.0 * t458 / t22195 * t496;
    let t22204 = t457 * t2750 * t496;
    let t22208 = t2751 * t2747;
    let t22210 = t2754 * t2747;
    let t22212 = t2757 * t2747;
    let t22215 = 480.0 * t7544 * t1101;
    (t22186, t22189, t22191, t22193, t22199, t22204, t22208, t22210, t22212, t22215)
}
