//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1107/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1107<F: Float>(t1005: F, t6996: F, t10871: F, t4238: F, t948: F, t969: F, t1410: F, t9099: F, t3477: F, t3514: F, t4244: F, t6951: F) -> (F, F, F, F, F, F) {
    let t10872 = t6996 * t1005;
    let t10873 = t10871 * t10872;
    let t10876 = t4238 * t948;
    let t10878 = F::cast_from(1.0_f64) * t10876 * t969;
    let t10880 = F::cast_from(2.0_f64) * t9099 * t1410;
    let t10882 = F::cast_from(2.0_f64) * t3477 * t3514;
    let t10884 = F::cast_from(2.0_f64) * t6951 * t4244;
    (t10873, t10876, t10878, t10880, t10882, t10884)
}
