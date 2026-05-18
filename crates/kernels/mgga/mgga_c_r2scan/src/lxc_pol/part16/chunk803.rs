//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 803/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk803<F: Float>(t2148: F, t8066: F, t2147: F, t113: F, t7503: F, t2115: F, t2155: F, t6063: F, t7619: F, t537: F, t7624: F, t560: F, t921: F) -> (F, F, F, F, F, F, F) {
    let t8067 = t2148 * t8066;
    let t8069 = F::new(0.11643651550782197811e-1) * t2147 * t8067;
    let t8070 = t7503 * t113;
    let t8071 = t2115 * t8070;
    let t8073 = F::new(0.97574405393827830186e-2) * t2155 * t8071;
    let t8074 = t6063 * t7619;
    let t8076 = F::new(0.19514881078765566037e-1) * t2155 * t8074;
    let t8077 = t2115 * t537;
    let t8078 = t8077 * t7624;
    let t8080 = F::new(0.97574405393827830186e-2) * t2155 * t8078;
    let t8081 = t921 * t560;
    (t8069, t8070, t8071, t8073, t8076, t8080, t8081)
}
