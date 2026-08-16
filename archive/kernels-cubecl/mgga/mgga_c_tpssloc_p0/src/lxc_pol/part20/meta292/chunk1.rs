//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1502/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1502<F: Float>(t10213: F, t10216: F, t9288: F, t974: F, t3030: F, t990: F, t3032: F, t3129: F) -> (F, F, F, F, F) {
    let t10942 = t10213 * t10216;
    let t10943 = t10942 * t9288;
    let t10944 = t974 * t10943;
    let t10947 = t990 * t3030;
    let t10948 = t10947 * t3032;
    let t10949 = t10948 * t3129;
    (t10943, t10944, t10947, t10948, t10949)
}
