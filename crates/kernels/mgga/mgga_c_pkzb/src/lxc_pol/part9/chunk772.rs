//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 772/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk772<F: Float>(t1976: F, t5484: F, t722: F, t730: F, t1975: F, t712: F) -> (F, F, F) {
    let t5486 = t1976 * t5484 * t722;
    let t5488 = F::new(0.35089341735807877242e1) * t730 * t5486;
    let t5490 = F::new(1.0) / t1975 / t712;
    (t5486, t5488, t5490)
}
