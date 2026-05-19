//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 691/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk691<F: Float>(t322: F, t355: F, t368: F, t7458: F, t7457: F, t1967: F, t2109: F, t2113: F, t1988: F, t2104: F, t137: F, t839: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7459 = t355 * t322;
    let t7461 = t7458 * t368 * t7459;
    let t7462 = t7457 * t7461;
    let t7464 = t1967 * t2109;
    let t7465 = F::cast_from(0.37737710747524982482e-2_f64) * t7464;
    let t7466 = t1967 * t2113;
    let t7468 = t1988 * t2104;
    let t7469 = F::cast_from(0.15724046144802076034e-2_f64) * t7468;
    let t7470 = t137 * t839;
    (t7459, t7461, t7462, t7464, t7465, t7466, t7468, t7469, t7470)
}
