//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 870/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk870<F: Float>(t460: F, t5462: F, t3302: F, t3603: F, t1248: F, t3781: F, t487: F) -> (F, F, F, F) {
    let t5463 = t460 * t5462;
    let t5464 = t3302 * t3603;
    let t5465 = t5464 * t1248;
    let t5477 = t3781 * t487;
    (t5463, t5464, t5465, t5477)
}
