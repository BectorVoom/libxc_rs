//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1051/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1051<F: Float>(t2314: F, t4193: F, t847: F, t3418: F, t3422: F, t4180: F, t6669: F, t4154: F, t828: F, t1359: F, t3385: F, t4170: F, t4167: F, t2275: F, t4166: F, t10534: F, t10549: F, t6530: F, t6749: F, t8676: F, t8808: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10802 = t4193 * t2314;
    let t10803 = t10802 * t847;
    let t10806 = t3422 * t3418;
    let t10809 = t4180 * t6669;
    let t10810 = t10809 * t847;
    let t10817 = t4154 * t828;
    let t10820 = t1359 * t3385;
    let t10823 = t4170 * t828;
    let t10826 = t4167 * t828;
    let t10829 = t4166 * t2275;
    let t10830 = t10829 * t828;
    let t10838 = -t6749 + 0.22831111111111111111e-1 * t6530 + 0.45662222222222222221e-1 * t8676 - t8808 - 0.17123333333333333333e-1 * t10534 + 0.5137e-1 * t10549;
    (t10802, t10803, t10806, t10809, t10810, t10817, t10820, t10823, t10826, t10829, t10830, t10838)
}
