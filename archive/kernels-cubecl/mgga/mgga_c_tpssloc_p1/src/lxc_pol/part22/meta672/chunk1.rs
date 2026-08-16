//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2228/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2228<F: Float>(t13847: F, t17748: F, t2986: F, t17849: F, t2960: F, t5838: F, t698: F, t973: F, t5844: F, t4509: F, t5836: F, t10190: F, t17794: F) -> (F, F, F, F, F, F) {
    let t61294 = t2986 * t13847 * t17748;
    let t61307 = t2960 * t17849;
    let t61310 = t973 * t698 * t5838;
    let t61313 = t973 * t698 * t5844;
    let t61322 = t4509 * t5836;
    let t61327 = t2986 * t10190 * t17794;
    (t61294, t61307, t61310, t61313, t61322, t61327)
}
