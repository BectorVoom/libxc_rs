//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 651/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk651<F: Float>(t10516: F, t10519: F, t10522: F, t10529: F, t10536: F, t10539: F, t10542: F, t10545: F, t10549: F, t10551: F, t10554: F, t10559: F, t10599: F, t10603: F, t10611: F) -> F {
    let t12133 = -t10516 + t10519 - t10522 - t10529 + t10536 + t10539 + t10542 + t10545 - t10549 - t10551 + t10554 + t10559 + t10599 - t10603 - t10611;
    t12133
}
