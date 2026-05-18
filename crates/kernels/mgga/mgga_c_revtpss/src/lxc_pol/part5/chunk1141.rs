//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1141/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1141<F: Float>(t1310: F, t5920: F, t116: F, t5876: F, t4343: F, t4542: F, t2404: F, t5966: F, t14613: F, t162: F, t4403: F, t14312: F) -> (F, F, F, F, F, F) {
    let t18242 = t1310 * t5920;
    let t18245 = t5876 * t116;
    let t18253 = t4542 * t4343;
    let t18256 = t2404 * t5966;
    let t18259 = t14613 * t162;
    let t18261 = F::new(24.0) * t18259 * t4403;
    let t18262 = F::new(2.0) * t14312;
    (t18242, t18245, t18253, t18256, t18261, t18262)
}
