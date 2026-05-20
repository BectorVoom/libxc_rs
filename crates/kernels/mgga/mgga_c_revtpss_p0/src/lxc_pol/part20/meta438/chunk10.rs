//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1661/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1661<F: Float>(t300: F, t45173: F, t45218: F, t45251: F, t45293: F, t12596: F, t3531: F, t1196: F, t12552: F, t3523: F, t43753: F, t1188: F, t12485: F) -> (F, F, F, F) {
    let t45296 = t300 * (t45173 + t45218 + t45251 + t45293);
    let t45298 = F::cast_from(0.14035736694323150897e2_f64) * t3531 * t12596;
    let t45302 = F::cast_from(0.6233709278045326953e3_f64) * t1196 * t12552 * t43753 * t3523;
    let t45306 = F::cast_from(0.14035736694323150897e2_f64) * t1196 * t12485 * t43753 * t1188;
    (t45296, t45298, t45302, t45306)
}
