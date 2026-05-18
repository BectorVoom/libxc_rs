//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1284/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1284<F: Float>(t16048: F, t3299: F, t11922: F, t4895: F, t4892: F, t140: F, t4886: F, t1011: F, t3241: F, t4924: F, t12047: F, t15905: F) -> (F, F, F, F, F) {
    let t16052 = t3299 * t16048;
    let t16055 = t11922 * t4895;
    let t16057 = F::new(0.57165357490759649296e-3) * t4892 * t16055;
    let t16060 = t140 * t4886;
    let t16062 = t1011 * t16060 / F::new(432.0);
    let t16064 = t3241 * t4924 / F::new(162.0);
    let t16067 = t12047 * t15905;
    (t16052, t16057, t16062, t16064, t16067)
}
