//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1902/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1902<F: Float>(t28814: F, t689: F, t94669: F, t2435: F, t28902: F, t7515: F, t98308: F, t97962: F, t14110: F, t96463: F, t5775: F, t7492: F) -> (F, F, F, F, F, F, F) {
    let t102244 = t28814 * t689;
    let t102246 = F::cast_from(0.15421710918628844644e0_f64) * t94669 * t102244;
    let t102249 = t2435 * t28902;
    let t102253 = F::cast_from(0.14456046980341999104e-1_f64) * t98308 * t7515;
    let t102255 = F::cast_from(0.25702851531048074406e-1_f64) * t97962 * t7515;
    let t102257 = t96463 * t14110;
    let t102261 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t7492 * t5775;
    (t102244, t102246, t102249, t102253, t102255, t102257, t102261)
}
