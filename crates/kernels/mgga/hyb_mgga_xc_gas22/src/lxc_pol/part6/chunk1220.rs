//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1220/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1220<F: Float>(t10350: F, t10424: F, t1270: F, t1282: F, t172: F, t180: F, t184: F, t2104: F, t2111: F, t2116: F, t2144: F, t24320: F, t28459: F, t28476: F, t28505: F, t28538: F, t28576: F, t3227: F, t3235: F, t3252: F, t3264: F, t4046: F, t4079: F, t6363: F, t740: F, t742: F, t756: F, t8354: F, t8431: F) -> (F,) {
    let t28585 = 14.0 * t3252 * t28476 - t24320 * t28476 - 24.0 * t6363 * t3235 * t3227 + 2.0 * t2104 * t4079 + 4.0 * t740 * t10424 + 2.0 * t4046 * t2144 + 4.0 * t8354 * t1282 + 8.0 * t3227 * t3264 + 4.0 * t1270 * t8431 + 2.0 * t28459 * t184 + 4.0 * t10350 * t756 + 2.0 * t172 * (t28538 + t28576) - t742 * t28459 - t2111 * t28505 * t180 + 4.0 * t2116 * t28505;
    (t28585,)
}
