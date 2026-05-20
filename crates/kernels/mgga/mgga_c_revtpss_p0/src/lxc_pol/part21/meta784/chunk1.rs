//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2822/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2822<F: Float>(t2832: F, t890: F, t11064: F, t14353: F, t14436: F, t1940: F, t2403: F, t2408: F, t2430: F, t41161: F, t4537: F, t4556: F, t50887: F, t50889: F, t50891: F, t50892: F, t50894: F, t50897: F, t50898: F) -> F {
    let t51792 = t890 * t2832;
    let t51802 = F::new(6.0) * t11064 * t1940 * t2408 * t4537 + F::new(9.0) * t14353 * t2403 * t2430 + F::new(6.0) * t14436 * t1940 * t51792 - F::new(9.0) * t2403 * t41161 * t4556 + t50887 - t50889 + t50891 + t50892 + t50894 + t50897 + t50898;
    t51802
}
