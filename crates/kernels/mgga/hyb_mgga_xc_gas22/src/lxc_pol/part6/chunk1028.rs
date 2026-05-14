//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1028/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1028<F: Float>(t4051: F, t6359: F, t180: F, t4046: F, t6383: F, t2124: F, t2132: F, t6394: F, t1270: F, t181: F, t178: F, t10350: F, t173: F, t3227: F, t3245: F, t3246: F, t3252: F, t3255: F, t3258: F, t4052: F, t747: F, t751: F, t8373: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10364 = t6359 * t4051;
    let t10373 = t180 * t4046;
    let t10394 = t6383 * t4051;
    let t10397 = t2124 * t4046;
    let t10403 = t2132 * t4046;
    let t10408 = t6394 * t4051;
    let t10411 = t1270 * t181;
    let t10414 = t178 * t1270;
    let t10424 = 15.0 / 2.0 * t4052 * t3246 - 4.0 * t3245 * t8373 - 5.0 / 2.0 * t10394 * t3246 - 2.0 * t10397 * t3246 + t747 * t10350 * t180 / 2.0 + t10403 * t3246 / 4.0 + t3252 * t8373 / 2.0 + t10408 * t3246 / 8.0 - 8.0 * t10411 * t3227 - 2.0 * t10414 * t8373 - 4.0 * t3255 * t4046 - t3258 * t10373 - 4.0 * t751 * t10350 - t173 * t10350 * t180;
    (t10364, t10373, t10394, t10397, t10403, t10408, t10411, t10414, t10424)
}
