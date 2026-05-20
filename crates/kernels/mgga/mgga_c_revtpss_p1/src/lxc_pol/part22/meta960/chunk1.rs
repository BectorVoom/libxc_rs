//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3222/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3222<F: Float>(t18281: F, t706: F, t750: F, t39737: F, t190: F, t60754: F, t18838: F, t892: F, t11075: F, t14375: F, t18435: F, t198: F, t2403: F, t2404: F, t39540: F, t39741: F, t39744: F, t39747: F, t39750: F, t39756: F, t4541: F, t5962: F, t775: F) -> (F, F, F, F) {
    let t61130 = t706 * t750 * t18281;
    let t61131 = F::new(8.0) * t61130;
    let t61135 = F::new(8.0) * t39737;
    let t61138 = F::new(4.0) * t706 * t190 * t60754;
    let t61139 = t18838 * t892;
    let t61146 = F::new(3.0) * t11075 * t2403 * t5962 + F::new(6.0) * t14375 * t198 * t5962 + F::new(12.0) * t18435 * t2404 * t4541 + F::new(6.0) * t2403 * t61139 * t775 - t39540 + t39741 + t39744 + t39747 + t39750 + t39756 + t61131 + t61135 + t61138;
    (t61131, t61135, t61138, t61146)
}
