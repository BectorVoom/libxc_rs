//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 670/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk670<F: Float>(t10795: F, t747: F, t1: F, t10215: F, t106: F, t192: F, t10496: F, t540: F, t10488: F, t1457: F, t10600: F, t1415: F, t31585: F, t493: F, t26126: F, t544: F) -> (F, F, F, F, F, F, F) {
    let t34013 = t10795 * t747;
    let t34131 = t10215 * t1 * t106 * t192;
    let t34157 = t10496 * t540;
    let t34223 = t1457 * t10488;
    let t34264 = t1415 * t10600;
    let t34267 = t493 * t31585;
    let t34286 = t544 * t26126;
    (t34013, t34131, t34157, t34223, t34264, t34267, t34286)
}
