//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2504/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2504<F: Float>(t12875: F, t12916: F, t5331: F, t12871: F, t5340: F, t1222: F, t12282: F, t17471: F, t1261: F, t12944: F, t3172: F, t12932: F, t3711: F) -> (F, F, F, F, F) {
    let t44773 = t5331 * t12916 * t12875;
    let t44776 = t5340 * t12916 * t12871;
    let t44786 = t1222 * t17471 * t12282;
    let t44789 = t1261 * t3172 * t12944;
    let t44792 = t3711 * t3172 * t12932;
    (t44773, t44776, t44786, t44789, t44792)
}
