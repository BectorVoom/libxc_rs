//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1086/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1086<F: Float>(t2958: F, t7291: F, t590: F, t2101: F, t21488: F, t320: F, t10639: F, t6058: F, t10736: F, t21497: F, t1897: F, t29190: F, t2936: F, t10704: F, t1850: F, t10636: F, t5227: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32607 = t2958 * t7291;
    let t32608 = t590 * t32607;
    let t32610 = 0.20508069947045931422e-1 * t21488 * t320 * t2101 * t32608;
    let t32613 = t590 * t10639;
    let t32615 = 0.10254034973522965711e-1 * t21488 * t320 * t6058 * t32613;
    let t32616 = t590 * t10736;
    let t32618 = 0.34180116578409885704e-2 * t21497 * t32616;
    let t32621 = 0.46143157380853345702e-1 * t1897 * t2936 * t29190;
    let t32622 = t1850 * t10704;
    let t32623 = 0.85450291446024714264e-3 * t32622;
    let t32625 = 0.17090058289204942853e-2 * t5227 * t10636;
    (t32608, t32610, t32613, t32615, t32616, t32618, t32621, t32623, t32625)
}
