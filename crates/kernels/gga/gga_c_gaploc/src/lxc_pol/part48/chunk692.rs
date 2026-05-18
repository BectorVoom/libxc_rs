//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 692/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk692<F: Float>(t13302: F, t13331: F, t209: F, t11288: F, t921: F, t1016: F, t10283: F, t3366: F, t8045: F, t2798: F, t3418: F, t3553: F, t6556: F) -> (F, F, F, F, F, F, F) {
    let t13332 = t13302 + t13331;
    let t13333 = t13332 * t209;
    let t13334 = t11288 * t921;
    let t13336 = F::new(2.0) * t10283 * t1016;
    let t13338 = F::new(4.0) * t8045 * t3366;
    let t13340 = F::new(2.0) * t2798 * t3418;
    let t13342 = F::new(2.0) * t6556 * t3553;
    (t13332, t13333, t13334, t13336, t13338, t13340, t13342)
}
