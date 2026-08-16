//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1330/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1330<F: Float>(t2490: F, t2494: F, t2538: F, t268: F, t675: F, t9310: F, t9314: F) -> (F, F, F, F) {
    let t39959 = t2490 * t2490;
    let t39960 = F::cast_from(1.0_f64) / t39959;
    let t39962 = t2494 * t2494;
    let t39963 = F::cast_from(1.0_f64) / t39962;
    let t39967 = t2538 * t2538;
    let t39989 = F::cast_from(0.3684616320282908548e2_f64) * t268 * t675 * t9310 * t9314;
    (t39960, t39963, t39967, t39989)
}
