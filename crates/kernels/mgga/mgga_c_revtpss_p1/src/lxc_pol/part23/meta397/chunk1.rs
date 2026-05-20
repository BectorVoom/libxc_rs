//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1757/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1757<F: Float>(t12268: F, t3617: F, t3708: F, t5265: F, t1260: F, t5326: F) -> (F, F, F) {
    let t17550 = t3617 * t12268;
    let t17556 = F::cast_from(0.28582678745379824648e-3_f64) * t3708 * t5265;
    let t17569 = t5326 * t1260;
    (t17550, t17556, t17569)
}
