//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2871/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2871<F: Float>(t51889: F, t51919: F, t51949: F, t51975: F, t52009: F, t52043: F, t52118: F, t52134: F, t964: F, t973: F, t981: F, t11467: F, t1633: F, t41235: F, t41238: F) -> (F, F, F) {
    let t52137 = t51889 + t51919 + t51949 + t51975 + t52009 + t52043 + t52118 + t52134;
    let t52141 = F::cast_from(0.5848223622634646207e0_f64) * t981 * t964 * t52137 * t973;
    let t52146 = F::cast_from(0.91082604192152556044e5_f64) * t981 * t41235 * t1633 * t41238 * t11467;
    (t52137, t52141, t52146)
}
