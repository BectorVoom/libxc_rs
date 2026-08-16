//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 464/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk464<F: Float>(t576: F, t580: F, t15: F, t22: F, t11: F, t14: F, t584: F, t588: F, t20: F, t27: F, t12: F, t19: F) -> (F, F, F, F, F, F, F) {
    let t2221 = F::cast_from(8.0_f64) * t576 * t580;
    let t2223 = F::cast_from(6.0_f64) * t15 * t22;
    let t2224 = t11 * t14;
    let t2226 = F::cast_from(12.0_f64) * t2224 * t22;
    let t2228 = F::cast_from(32.0_f64) * t584 * t588;
    let t2230 = F::cast_from(20.0_f64) * t20 * t27;
    let t2231 = t12 * t19;
    (t2221, t2223, t2224, t2226, t2228, t2230, t2231)
}
