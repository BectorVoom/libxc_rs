//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1099/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1099<F: Float>(t28166: F, t8567: F, t28168: F, t32117: F, t7898: F, t28187: F, t8568: F, t33913: F, t7239: F, t33597: F, t7235: F, t32110: F, t7732: F) -> (F, F, F, F, F, F) {
    let t125496 = t8567 * t28166;
    let t125497 = t125496 * t28168;
    let t125499 = t7898 * t32117;
    let t125500 = t8568 * t28187;
    let t125502 = t33913 * t7239;
    let t125505 = F::new(3.0) * t7235 * t33597;
    let t125507 = F::new(2.0) * t7732 * t32110;
    (t125497, t125499, t125500, t125502, t125505, t125507)
}
