//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2975/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2975<F: Float>(t52196: F, t52201: F, t52204: F, t52207: F, t52209: F, t52211: F, t52213: F, t52216: F, t52218: F, t52221: F, t52223: F, t52226: F, t52229: F, t52231: F, t52235: F, t52237: F, t52242: F, t52245: F, t52860: F, t52863: F) -> F {
    let t54231 = -t52196 - t52201 - t52204 - t52207 - t52209 + t52211 - t52213 + t52216 + t52218 + t52221 + t52223 + t52226 + t52229 + t52231 + t52235 + t52237 + t52242 + t52245 + t52860 + t52863;
    t54231
}
