//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 989/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk989<F: Float>(t11249: F, t13045: F, t13040: F, t3597: F, t13036: F, t3603: F, t1244: F, t471: F, t3367: F, t414: F, t66: F, t11239: F, t1243: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13046 = t11249 * t13045;
    let t13051 = t3597 * t13040;
    let t13052 = t13036 * t13051;
    let t13053 = t11249 * t3603;
    let t13061 = t1244 * t13040;
    let t13062 = t13036 * t13061;
    let t13063 = t11249 * t471;
    let t13099 = F::new(1.0) / t414 / t3367;
    let t13100 = t66 * t13099;
    let t13126 = t11239 * t1243;
    (t13046, t13051, t13052, t13053, t13061, t13062, t13063, t13099, t13100, t13126)
}
