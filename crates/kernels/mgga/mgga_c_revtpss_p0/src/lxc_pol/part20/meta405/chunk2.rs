//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1500/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1500<F: Float>(t283: F, t2852: F, t66: F, t11951: F, t3211: F, t1025: F, t3218: F, t371: F, t676: F, t11804: F, t11921: F, t247: F, t4837: F) -> (F, F, F, F) {
    let t42471 = F::new(1.0) / t283 / t2852;
    let t42472 = t66 * t42471;
    let t42477 = t3211 * t11951;
    let t42481 = t1025 * t371 * t676 * t3218;
    let t42487 = t4837 * t247 * t11921 * t11804;
    (t42472, t42477, t42481, t42487)
}
