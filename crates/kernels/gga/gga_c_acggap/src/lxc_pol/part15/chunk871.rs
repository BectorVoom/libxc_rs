//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 871/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk871<F: Float>(t13462: F, t2065: F, t2450: F, t56: F, t30321: F, t1581: F, t7614: F, t2327: F, t7780: F, t30325: F, t30318: F, t532: F, t1569: F, t1988: F, t8838: F, t1459: F, t1980: F, t33883: F, t7458: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34278 = t2450 * t2065 * t56 * t13462;
    let t34283 = 0.42874018118069736972e-3 * t30321;
    let t34284 = t7614 * t1581;
    let t34286 = t7780 * t2327;
    let t34288 = 0.18868855373762491241e-2 * t30325;
    let t34293 = t30318 * t532;
    let t34295 = t7614 * t1569;
    let t34297 = t1988 * t8838;
    let t34305 = t1980 * t7458 * t1459 * t33883;
    (t34278, t34283, t34284, t34286, t34288, t34293, t34295, t34297, t34305)
}
