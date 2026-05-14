//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 757/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk757<F: Float>(t42587: F, t10166: F, t9074: F, t9086: F, t1063: F, t3148: F, t7974: F, t2268: F, t32005: F, t894: F, t41596: F, t426: F, t535: F, t39671: F, t39674: F, t39677: F) -> (F, F, F, F, F, F, F, F) {
    let t42588 = 0.71137516589190373998e-2 * t42587;
    let t42590 = t9074 * t10166 * t9086;
    let t42591 = 0.71137516589190373998e-2 * t42590;
    let t42594 = 0.28455006635676149599e-1 * t1063 * t3148 * t7974;
    let t42597 = 0.56910013271352299198e-1 * t2268 * t894 * t32005;
    let t42601 = 0.28455006635676149599e-1 * t2268 * t535 * t41596 * t426;
    let t42602 = 0.284550066356761496e-1 * t39671;
    let t42603 = 0.142275033178380748e-1 * t39674;
    let t42604 = 0.23712505529730124666e-2 * t39677;
    (t42588, t42591, t42594, t42597, t42601, t42602, t42603, t42604)
}
