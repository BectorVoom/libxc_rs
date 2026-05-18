//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1028/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1028<F: Float>(t30318: F, t532: F, t1569: F, t7614: F, t1988: F, t8838: F, t1089: F, t1459: F, t33878: F, t598: F, t1980: F, t33883: F, t7458: F) -> (F, F, F, F, F) {
    let t34293 = t30318 * t532;
    let t34295 = t7614 * t1569;
    let t34297 = t1988 * t8838;
    let t34301 = t598 * t1089 * t1459 * t33878;
    let t34305 = t1980 * t7458 * t1459 * t33883;
    (t34293, t34295, t34297, t34301, t34305)
}
