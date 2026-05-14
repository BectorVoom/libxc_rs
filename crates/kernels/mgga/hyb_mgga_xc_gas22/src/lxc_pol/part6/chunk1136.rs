//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1136/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1136<F: Float>(t1230: F, t1861: F, t10: F, t16: F, t6610: F, t2951: F, t639: F, t7834: F, t1806: F, t92: F, t2970: F, t7837: F, t7884: F, t8172: F, t1181: F, t5885: F) -> (F, F, F, F, F, F, F, F) {
    let t23023 = t1230 * t1861;
    let t23029 = t6610 * t10 * t16;
    let t23030 = t23029 * t2951;
    let t23043 = t7834 * t639;
    let t23048 = t6610 * t1806 * t92;
    let t23050 = t2970 * t23048 * t7837;
    let t23083 = t7884 * t8172;
    let t23085 = t1181 * t5885;
    (t23023, t23029, t23030, t23043, t23048, t23050, t23083, t23085)
}
