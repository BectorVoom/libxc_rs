//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1067/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1067<F: Float>(t1904: F, t3895: F, t2439: F, t213: F, t5710: F, t1532: F, t2609: F, t2398: F, t4305: F, t177: F, t4392: F, t762: F) -> (F, F, F, F, F) {
    let t14296 = t3895 * t1904;
    let t14297 = t2439 * t14296;
    let t14299 = t213 * t5710;
    let t14312 = t1532 * t2609;
    let t14317 = F::new(8.0) * t2398 * t4305;
    let t14322 = t4392 * t177;
    let t14324 = F::new(0.11696447245269292414e1) * t14322 * t762;
    (t14297, t14299, t14312, t14317, t14324)
}
