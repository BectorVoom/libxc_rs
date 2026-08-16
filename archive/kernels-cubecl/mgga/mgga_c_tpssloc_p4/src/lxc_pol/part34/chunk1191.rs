//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1191/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1191<F: Float>(t26233: F, t6417: F, t20492: F, t80903: F, t20497: F, t22761: F, t20512: F, t80830: F, t1998: F, t20416: F, t236: F, t6926: F) -> (F, F, F, F, F) {
    let t107088 = t26233 * t6417;
    let t107090 = t80903 * t20492;
    let t107093 = t22761 * t20497;
    let t107096 = t80830 * t20512;
    let t107100 = t6926 * t1998 * t236 * t20416;
    (t107088, t107090, t107093, t107096, t107100)
}
